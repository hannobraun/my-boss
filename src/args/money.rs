use clap::Clap;

#[derive(Clap)]
pub enum Command {
    /// Show report
    Report(Report),
}

#[derive(Clap)]
pub struct Report;
