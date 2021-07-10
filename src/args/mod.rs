pub mod contacts;

use clap::Clap;

use self::contacts::ContactsCmd;

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
