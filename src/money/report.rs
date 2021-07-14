use std::io::{self, Write as _};

use indexmap::IndexSet;
use tabwriter::TabWriter;
use termcolor::{Ansi, Color, ColorSpec, WriteColor as _};

use super::transactions::{Accounts, Amount, Transactions};

pub fn write_report(
    transactions: Transactions,
    writer: impl io::Write,
) -> anyhow::Result<()> {
    let writer = TabWriter::new(writer);
    let mut writer = Ansi::new(writer);

    let mut budgets = AccountNames::new();

    for transaction in &transactions {
        budgets.collect_names(&transaction.budgets);
    }

    // Write header
    writer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_intense(true)
            .set_bold(true),
    )?;
    write!(writer, "Date\tDescription\tAmount\tBudgets")?;
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
    write!(writer, "\t\t\t")?;
    budgets.write_header(&mut writer)?;
    writer.reset()?;
    writeln!(writer)?;

    // Write transactions
    for transaction in &transactions {
        write!(
            writer,
            "{}\t{}\t",
            transaction.date, transaction.description
        )?;

        write_amount(&transaction.amount, false, &mut writer)?;
        write!(writer, "\t")?;
        write_amounts(&transaction.budgets, &budgets, &mut writer)?;

        writer.reset()?;
        writeln!(writer)?;
    }

    // Write totals
    writer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_intense(true)
            .set_bold(true),
    )?;
    write!(writer, "\tTotals\t")?;
    write_amount(&transactions.total(), true, &mut writer)?;
    write!(writer, "\t")?;
    budgets.write_totals(&transactions, &mut writer)?;
    writer.reset()?;
    writeln!(writer)?;

    writer.flush()?;

    Ok(())
}

fn write_amounts(
    accounts: &Accounts,
    names: &AccountNames,
    writer: &mut Ansi<impl io::Write>,
) -> anyhow::Result<()> {
    for name in &names.0 {
        if let Some(amount) = accounts.amount_for(name) {
            write_amount(&amount, false, writer)?;
        }
        write!(writer, "\t")?;
    }

    Ok(())
}

fn write_amount(
    amount: &Amount,
    highlight: bool,
    writer: &mut Ansi<impl io::Write>,
) -> anyhow::Result<()> {
    let color = if amount.is_negative() {
        Color::Red
    } else {
        Color::Green
    };

    writer.set_color(
        ColorSpec::new()
            .set_fg(Some(color))
            .set_intense(highlight)
            .set_bold(highlight),
    )?;
    write!(writer, "{}", amount)?;

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

    fn write_totals(
        &self,
        transactions: &Transactions,
        writer: &mut Ansi<impl io::Write>,
    ) -> anyhow::Result<()> {
        for name in &self.0 {
            let amount = transactions.account_total(name);
            write_amount(&amount, true, writer)?;

            write!(writer, "\t")?;
        }

        Ok(())
    }
}
