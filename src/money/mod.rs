mod amount;
mod import;
mod report;
mod store;
mod transactions;

// TASK: Add `allocation` module that contains an `Allocation` struct. An
//       allocation refers back to a transaction, and allocates its amount to a
//       number of budgets.
//
//       The core transaction logic doesn't need to know about this, but the
//       report needs to be aware of them, to work them into the report.

use std::{io, path::Path};

use anyhow::Context as _;
use walkdir::WalkDir;

use self::transactions::{Transaction, Transactions};

#[derive(Clone, Debug)]
pub struct Money(Transactions);

impl Money {
    /// Import transactions from CSV file
    pub fn import(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let transactions =
            import::from_csv(path.as_ref()).with_context(|| {
                format!("Error importing from `{}`", path.as_ref().display())
            })?;
        Ok(Self(transactions.into()))
    }

    /// Load all transactions in a directory
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut transactions = Vec::new();

        for entry in WalkDir::new(path) {
            let entry = entry?;

            if entry.file_type().is_dir() {
                continue;
            }

            let transaction =
                Transaction::load(entry.path()).with_context(|| {
                    format!(
                        "Failed to load contact from `{}`",
                        entry.path().display()
                    )
                })?;
            transactions.push(transaction);
        }

        transactions.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(Self(transactions.into()))
    }

    /// Print a report to stdout
    pub fn report(&self, writer: impl io::Write) -> anyhow::Result<()> {
        report::write_report(&self.0, writer)
    }

    /// Store transactions to TOML files
    pub fn store(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        store::to_toml(&self.0, path)
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
