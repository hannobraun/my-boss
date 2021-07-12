use std::io::{self, Write as _};

use indexmap::IndexSet;
use tabwriter::TabWriter;
use termcolor::{Ansi, Color, ColorSpec, WriteColor as _};

use super::transaction::{Accounts, Amount, Transaction};

pub fn write_report(
    transactions: &[Transaction],
    writer: impl io::Write,
) -> anyhow::Result<()> {
    let writer = TabWriter::new(writer);
    let mut writer = Ansi::new(writer);

    let mut accounts = AccountNames::new();
    let mut budgets = AccountNames::new();

    for transaction in transactions.clone().into_iter() {
        accounts.collect_names(&transaction.accounts);
        budgets.collect_names(&transaction.budgets);
    }

    // Write header
    writer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_intense(true)
            .set_bold(true),
    )?;
    write!(writer, "Date\tDescription\tAccounts")?;
    accounts.reserve_header_space(&mut writer)?;
    write!(writer, "Budgets")?;
    budgets.reserve_header_space(&mut writer)?;
    writer.reset()?;
    writeln!(writer)?;

    // Write sub-header
    writer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_intense(true)
            .set_bold(true),
    )?;
    write!(writer, "\t\t")?;
    accounts.write_header(&mut writer)?;
    budgets.write_header(&mut writer)?;
    writer.reset()?;
    writeln!(writer)?;

    // Write transactions
    // TASK: Print transactions sorted by date.
    for transaction in transactions.into_iter() {
        write!(
            writer,
            "{}\t{}\t",
            transaction.date, transaction.description
        )?;

        write_accounts(&transaction.accounts, &accounts, &mut writer)?;
        write_accounts(&transaction.budgets, &budgets, &mut writer)?;

        writer.reset()?;
        writeln!(writer)?;
    }

    // TASK: Write last line with totals.

    writer.flush()?;

    Ok(())
}

fn write_amount(
    amount: &Amount,
    writer: &mut Ansi<impl io::Write>,
) -> anyhow::Result<()> {
    let color = if amount.is_negative() {
        Color::Red
    } else {
        Color::Green
    };

    writer.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(writer, "{}", amount)?;

    Ok(())
}

fn write_accounts(
    accounts: &Accounts,
    names: &AccountNames,
    writer: &mut Ansi<impl io::Write>,
) -> anyhow::Result<()> {
    for name in &names.0 {
        if let Some(amount) = accounts.amount_for(name) {
            write_amount(&amount, writer)?;
        }
        write!(writer, "\t")?;
    }

    Ok(())
}

struct AccountNames(IndexSet<String>);

impl AccountNames {
    fn new() -> Self {
        Self(IndexSet::new())
    }

    fn collect_names(&mut self, accounts: &Accounts) {
        for name in accounts.names() {
            self.0.insert(name.clone());
        }
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
