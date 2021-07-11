use std::{
    fmt,
    io::{self, Write as _},
    path::Path,
};

use anyhow::{anyhow, Context as _};
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use tabwriter::TabWriter;
use term::{Terminal as _, TerminfoTerminal};
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
        let writer = TabWriter::new(writer);
        let mut writer = TerminfoTerminal::new(writer)
            .ok_or_else(|| anyhow!("Failed to initialize terminfo terminal"))?;

        let mut accounts = AccountNames::new();
        let mut budgets = AccountNames::new();

        for transaction in &self.0 {
            transaction.accounts.collect_names_into(&mut accounts);
            transaction.budgets.collect_names_into(&mut budgets);
        }

        // Write header
        writer.attr(term::Attr::Bold)?;
        writer.fg(term::color::BRIGHT_WHITE)?;
        write!(writer, "Date\tDescription\tAccounts")?;
        accounts.reserve_header_space(&mut writer)?;
        write!(writer, "Budgets")?;
        budgets.reserve_header_space(&mut writer)?;
        writer.reset()?;
        writeln!(writer)?;

        // Write sub-header
        writer.attr(term::Attr::Bold)?;
        writer.fg(term::color::BRIGHT_WHITE)?;
        write!(writer, "\t\t")?;
        accounts.write_header(&mut writer)?;
        budgets.write_header(&mut writer)?;
        writer.reset()?;
        writeln!(writer)?;

        // Write transactions
        for transaction in &self.0 {
            write!(
                writer,
                "{}\t{}\t",
                transaction.date, transaction.description
            )?;
            transaction.accounts.write(&accounts, &mut writer)?;
            transaction.budgets.write(&budgets, &mut writer)?;
            writer.reset()?;
            writeln!(writer)?;
        }

        // TASK: Write last line with totals.

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
    fn collect_names_into(&self, names: &mut AccountNames) {
        for name in self.0.keys() {
            names.0.insert(name.clone());
        }
    }

    fn write(
        &self,
        names: &AccountNames,
        writer: &mut TerminfoTerminal<impl io::Write>,
    ) -> anyhow::Result<()> {
        for name in &names.0 {
            if let Some(amount) = self.0.get(name.as_str()) {
                amount.write(writer)?;
            }
            write!(writer, "\t")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Amount(i64);

impl Amount {
    fn write(
        &self,
        writer: &mut TerminfoTerminal<impl io::Write>,
    ) -> anyhow::Result<()> {
        if self.0.is_negative() {
            writer.fg(term::color::RED)?;
        } else {
            writer.fg(term::color::GREEN)?;
        }

        write!(writer, "{}", self)?;

        Ok(())
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}€", self.0 / 100, self.0.abs() % 100)
    }
}

struct AccountNames(IndexSet<String>);

impl AccountNames {
    fn new() -> Self {
        Self(IndexSet::new())
    }

    fn reserve_header_space(
        &self,
        mut writer: impl io::Write,
    ) -> anyhow::Result<()> {
        for _ in 0..self.0.len() {
            write!(writer, "\t")?;
        }

        Ok(())
    }

    fn write_header(&self, mut writer: impl io::Write) -> anyhow::Result<()> {
        for name in &self.0 {
            write!(writer, "{}\t", name)?;
        }

        Ok(())
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
