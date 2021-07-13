mod import;
mod report;
mod transactions;

use std::{io, path::Path};

use anyhow::Context as _;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use self::transactions::{Transaction, Transactions};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Money(pub Vec<Transaction>);

impl Money {
    pub fn import(
        input: impl AsRef<Path>,
        output: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        import::from_csv(input, output)
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
        report::write_report(Transactions::new(&self.0), writer)
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
