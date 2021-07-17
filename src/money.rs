mod import;
mod report;
mod transactions;

use std::{io, path::Path};

use anyhow::Context as _;
use walkdir::WalkDir;

use crate::config;

use self::transactions::{Transaction, Transactions};

#[derive(Clone, Debug)]
pub struct Money(Vec<Transaction>);

impl Money {
    pub fn import(
        input: impl AsRef<Path>,
        config: config::Money,
    ) -> anyhow::Result<()> {
        import::from_csv(input, config)
    }

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

        transactions.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(Self(transactions))
    }

    /// Print a report to stdout
    pub fn report(&self, writer: impl io::Write) -> anyhow::Result<()> {
        report::write_report(Transactions::new(self.0.clone()), writer)
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
