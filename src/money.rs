mod report;

use std::{fmt, io, path::Path};

use anyhow::Context as _;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
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

    /// Print a report to stdout
    pub fn report(&self, writer: impl io::Write) -> anyhow::Result<()> {
        report::write_report(&self.0, writer)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    /// The date the transaction occurred
    pub date: Date,

    /// Description of the transaction
    pub description: String,

    /// The accounts the transaction affected
    pub accounts: Accounts,

    /// The budgets the transaction affected
    pub budgets: Accounts,
}

impl Transaction {
    /// Load a transaction
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        load(path)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Accounts(IndexMap<String, Amount>);

impl Accounts {
    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    pub fn amount_for(&self, name: impl AsRef<str>) -> Option<Amount> {
        self.0.get(name.as_ref()).copied()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Amount(i64);

impl Amount {
    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}€", self.0 / 100, self.0.abs() % 100)
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
