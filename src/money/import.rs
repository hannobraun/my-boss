use std::{
    fs::File,
    io::{prelude::*, Cursor},
    path::Path,
};

use anyhow::{anyhow, bail};
use encoding::{all::ISO_8859_1, decode, DecoderTrap};
use regex::Regex;

use crate::money::transactions::Amount;

pub fn from_csv(
    input: impl AsRef<Path>,
    _output: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    File::open(input)?.read_to_end(&mut buf)?;

    let input = decode(&buf, DecoderTrap::Strict, ISO_8859_1)
        .0
        .map_err(|err| anyhow!("Error decoding CSV file: {}", err))?;

    // Filter out metadata before the actual CSV data.
    let (_, input) = input
        .split_once("\n\"Buchungstag\"")
        .ok_or_else(|| anyhow!("Failed to find start of CSV data"))?;

    // Filter out metadata after the actual CSV data.
    let (input, _) = input
        .rsplit_once("\n")
        .ok_or_else(|| anyhow!("Failed to drop last line (1)"))?;
    let (input, _) = input
        .rsplit_once("\n")
        .ok_or_else(|| anyhow!("Failed to drop last line (2)"))?;
    let (input, _) = input
        .rsplit_once("\n")
        .ok_or_else(|| anyhow!("Failed to drop last line (3)"))?;

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .quote(b'"')
        .flexible(true)
        .from_reader(Cursor::new(input));

    for record in reader.records() {
        let record = record?;

        let date = record
            .get(0)
            .ok_or_else(|| anyhow!("Could not read date"))?;
        // TASK: Build more extensive description.
        let description = record
            .get(3)
            .ok_or_else(|| anyhow!("Could not read description"))?;
        let amount = record
            .get(11)
            .ok_or_else(|| anyhow!("Could not read amount"))?;
        let credit_or_debit = record
            .get(12)
            .ok_or_else(|| anyhow!("Could not read credit/debit"))?;

        let amount = parse_amount(amount, credit_or_debit)?;

        dbg!((date, description, amount, credit_or_debit));

        // TASK: Create `Transaction`.
        // TASK: Store `Transaction` in file.
    }

    Ok(())
}

fn parse_amount(amount: &str, credit_or_debit: &str) -> anyhow::Result<Amount> {
    let regex = Regex::new(r"(\d+),(\d{2})")?;
    let captures = regex
        .captures(amount)
        .ok_or_else(|| anyhow!("Couldn't parse amount"))?;

    let amount_int = captures[1].parse()?;
    let amount_frac = captures[2].parse()?;

    let amount = Amount::from_int_and_frac(amount_int, amount_frac);
    let amount = match credit_or_debit {
        "H" => amount,
        "S" => -amount,
        credit_or_debit => {
            bail!("Failed to parse credit/debit: {}", credit_or_debit)
        }
    };

    Ok(amount)
}
