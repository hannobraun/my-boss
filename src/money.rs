use std::{
    io::{self, Write as _},
    path::Path,
};

use anyhow::Context as _;
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use tabwriter::TabWriter;
use time::Date;
use walkdir::WalkDir;

use crate::util::toml::load;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Money(pub Vec<Transaction>);

impl Money {
    /// Load all transactions in a directory
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut transactions = Vec::new();

        for entry in WalkDir::new(path) {
            let entry = entry?;

            if entry.file_type().is_dir() {
                continue;
            }

            let contact =
                Transaction::load(entry.path()).with_context(|| {
                    format!(
                        "Failed to load contact from `{}`",
                        entry.path().display()
                    )
                })?;
            transactions.push(contact);
        }

        Ok(Self(transactions))
    }

    /// Print a report to stout
    pub fn report(&self, writer: impl io::Write) -> anyhow::Result<()> {
        // TASK: Format money amounts correctly.
        // TASK: Improve formatting of headers and sub-headers (make them bold,
        //       for example).

        let mut writer = TabWriter::new(writer);

        let mut accounts = IndexSet::new();
        let mut budgets = IndexSet::new();

        for transaction in &self.0 {
            for account in transaction.accounts.keys() {
                accounts.insert(account);
            }
            for budget in transaction.budgets.keys() {
                budgets.insert(budget);
            }
        }

        // Write header
        write!(writer, "Date\tDescription\tAccounts")?;
        for _ in 0..accounts.len() {
            write!(writer, "\t")?;
        }
        write!(writer, "Budgets")?;
        for _ in 0..budgets.len() {
            write!(writer, "\t")?;
        }
        writeln!(writer)?;

        // Write sub-header
        write!(writer, "\t\t")?;
        for account in &accounts {
            write!(writer, "{}\t", account)?;
        }
        for budget in &budgets {
            write!(writer, "{}\t", budget)?;
        }
        writeln!(writer)?;

        for transaction in &self.0 {
            write!(
                writer,
                "{}\t{}\t",
                transaction.date, transaction.description
            )?;
            for account in &accounts {
                if let Some(amount) = transaction.accounts.get(account.as_str())
                {
                    write!(writer, "{}", amount)?;
                }
                write!(writer, "\t")?;
            }
            for account in &accounts {
                if let Some(amount) = transaction.accounts.get(account.as_str())
                {
                    write!(writer, "{}", amount)?;
                }
                write!(writer, "\t")?;
            }
            writeln!(writer)?;
        }

        writer.flush()?;

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    /// The date the transaction occurred
    pub date: Date,

    /// Description of the transaction
    pub description: String,

    /// The accounts the transaction affected
    pub accounts: IndexMap<String, i64>,

    /// The budgets the transaction affected
    pub budgets: IndexMap<String, i64>,
}

impl Transaction {
    /// Load a transaction
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        load(path)
    }
}

#[cfg(test)]
mod tests {
    use super::Money;

    #[test]
    fn transactions_should_match_example_files() {
        Money::load("money").unwrap();

        // Nothing to check, I think. It's enough that the previous call doesn't
        // cause an error.
    }
}
