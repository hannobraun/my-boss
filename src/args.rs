use std::path::PathBuf;

use clap::Clap;

#[derive(Clap)]
#[clap(name = "My Boss")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap)]
pub enum Command {
    /// Create a configuration file with default values
    Init,

    /// Manage contacts
    Contacts(ContactsCmd),
}

#[derive(Clap)]
pub enum ContactsCmd {
    /// Generate a new contact
    Generate(GenerateContact),

    /// List contacts. By default, only contacts where communication is due are
    /// listed.
    List(ListContacts),
}

#[derive(Clap)]
pub struct GenerateContact {
    /// The name of the new contact
    pub name: String,

    /// The path to the directory where the contact will be generated. If not
    /// specified, the path will be taken from My Boss's configuration file.
    #[clap(short, long)]
    pub path: Option<PathBuf>,
}

#[derive(Clap)]
pub struct ListContacts {
    /// List all contacts, not just those where communication is due.
    #[clap(short, long)]
    pub all: bool,
}
