use std::path::Path;

use anyhow::Context as _;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Date;
use walkdir::WalkDir;

use crate::util::toml::load;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transactions(pub Vec<Transaction>);

impl Transactions {
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
    use super::Transactions;

    #[test]
    fn transactions_should_match_example_files() {
        Transactions::load("money").unwrap();

        // Nothing to check, I think. It's enough that the previous call doesn't
        // cause an error.
    }
}
