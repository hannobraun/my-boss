use std::io::{self, Write as _};

use tabwriter::TabWriter;
use termcolor::{Ansi, Color, ColorSpec, WriteColor as _};

use super::{amount::Amount, transactions::Transactions};

pub fn write_report(
    transactions: &Transactions,
    writer: impl io::Write,
) -> anyhow::Result<()> {
    let writer = TabWriter::new(writer);
    let mut writer = Ansi::new(writer);

    // Write header
    writer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_intense(true)
            .set_bold(true),
    )?;
    write!(writer, "Date\tDescription\tAmount")?;
    writer.reset()?;
    writeln!(writer)?;

    // Write empty line below header
    writeln!(writer, "\t\t")?;

    // Write transactions
    for transaction in transactions {
        write!(
            writer,
            "{}\t{}\t",
            transaction.date, transaction.description
        )?;

        write_amount(&transaction.amount, false, &mut writer)?;
        write!(writer, "\t")?;

        writer.reset()?;
        writeln!(writer)?;
    }

    // Write empty line below transactions
    writeln!(writer, "\t\t")?;

    // Write totals
    writer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_intense(true)
            .set_bold(true),
    )?;
    write!(writer, "\tTotal\t")?;
    write_amount(&transactions.total(), true, &mut writer)?;
    writer.reset()?;
    writeln!(writer)?;

    writer.flush()?;

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
