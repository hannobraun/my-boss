use std::path::PathBuf;

#[derive(clap::Parser, Clone)]
pub enum Command {
    /// Create a new contact
    Create(Create),

    /// List contacts. By default, only contacts where communication is due are
    /// listed.
    List(List),
}

#[derive(clap::Parser, Clone)]
pub struct Create {
    /// The name of the new contact
    pub name: String,

    /// The path to the directory where the contact will be created. If not
    /// specified, the path will be taken from My Boss's configuration file.
    #[clap(short, long)]
    pub path: Option<PathBuf>,
}

#[derive(clap::Parser, Clone)]
pub struct List {
    /// List all contacts, not just those where communication is due.
    #[clap(short, long)]
    pub all: bool,
}
