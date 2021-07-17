use std::path::PathBuf;

use clap::Clap;

#[derive(Clap, Clone)]
pub enum Command {
    /// Import transactions from CSV file
    Import(Import),

    /// Show report
    Report(Report),
}

#[derive(Clap, Clone)]
pub struct Import {
    /// The CSV file to import transactions from
    pub file: PathBuf,
}

#[derive(Clap, Clone)]
pub struct Report;
