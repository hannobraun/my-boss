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
            transaction.accounts.collect_names_into(&mut accounts);
            transaction.budgets.collect_names_into(&mut budgets);
        }

        // Write header
        write!(writer, "Date\tDescription\tAccounts")?;
        Account::reserve_header_space(&mut writer, &accounts)?;
        write!(writer, "Budgets")?;
        Account::reserve_header_space(&mut writer, &budgets)?;
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
                if let Some(amount) =
                    transaction.accounts.0.get(account.as_str())
                {
                    write!(writer, "{}", amount.0)?;
                }
                write!(writer, "\t")?;
            }
            for budget in &budgets {
                if let Some(amount) = transaction.budgets.0.get(budget.as_str())
                {
                    write!(writer, "{}", amount.0)?;
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
    pub accounts: Account,

    /// The budgets the transaction affected
    pub budgets: Account,
}

impl Transaction {
    /// Load a transaction
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        load(path)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account(IndexMap<String, Amount>);

impl Account {
    fn collect_names_into(&self, names: &mut IndexSet<String>) {
        for name in self.0.keys() {
            names.insert(name.clone());
        }
    }

    fn reserve_header_space(
        mut writer: impl io::Write,
        names: &IndexSet<String>,
    ) -> anyhow::Result<()> {
        for _ in 0..names.len() {
            write!(writer, "\t")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Amount(i64);

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
