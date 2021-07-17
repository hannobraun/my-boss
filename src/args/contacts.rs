use std::path::PathBuf;

use clap::Clap;

#[derive(Clap, Clone)]
pub enum Command {
    /// Generate a new contact
    Generate(Generate),

    /// List contacts. By default, only contacts where communication is due are
    /// listed.
    List(List),
}

#[derive(Clap, Clone)]
pub struct Generate {
    /// The name of the new contact
    pub name: String,

    /// The path to the directory where the contact will be generated. If not
    /// specified, the path will be taken from My Boss's configuration file.
    #[clap(short, long)]
    pub path: Option<PathBuf>,
}

#[derive(Clap, Clone)]
pub struct List {
    /// List all contacts, not just those where communication is due.
    #[clap(short, long)]
    pub all: bool,
}
