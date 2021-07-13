use std::path::PathBuf;

use clap::Clap;

#[derive(Clap)]
pub enum Command {
    /// Import transactions from CSV file
    Import(Import),

    /// Show report
    Report(Report),
}

#[derive(Clap)]
pub struct Import {
    /// The CSV file to import transactions from
    pub file: PathBuf,
}

#[derive(Clap)]
pub struct Report;
