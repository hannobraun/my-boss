mod import;
mod report;
mod transactions;

use std::{
    fs::File,
    io::{self, prelude::*},
    path::Path,
};

use anyhow::Context as _;
use time::macros::format_description;
use walkdir::WalkDir;

use crate::config;

use self::transactions::{Transaction, Transactions};

#[derive(Clone, Debug)]
pub struct Money(Transactions);

impl Money {
    /// Import transactions from CSV file
    pub fn import(
        input: impl AsRef<Path>,
        config: config::Money,
    ) -> anyhow::Result<()> {
        let transactions = import::from_csv(input, config.clone())?;

        for transaction in &transactions {
            let date = transaction
                .date
                .format(&format_description!("[year]-[month]-[day]"))?;

            let mut i = 0;
            loop {
                let file_name = format!("{}_{}.toml", date, i);
                let path = config.path.join(&file_name);

                if path.exists() {
                    i += 1;
                    continue;
                }

                let transaction = toml::to_vec(&transaction)?;
                File::create(path)?.write_all(&transaction)?;
                break;
            }
        }

        Ok(())
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

        Ok(Self(transactions.into()))
    }

    /// Print a report to stdout
    pub fn report(&self, writer: impl io::Write) -> anyhow::Result<()> {
        report::write_report(&self.0, writer)
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
