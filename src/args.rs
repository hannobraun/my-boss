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
    Generate(GenerateContact),
    List(ListContacts),
}

#[derive(Clap)]
pub struct GenerateContact {
    pub name: String,
    // TASK: Add argument to override path.
}

#[derive(Clap)]
pub struct ListContacts {
    #[clap(short, long)]
    pub all: bool,
}
