pub mod contacts;

use clap::Clap;

#[derive(Clap, Clone)]
#[clap(name = "My Boss")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap, Clone)]
pub enum Command {
    /// Create a configuration file with default values
    Init,

    /// Manage contacts
    Contacts(contacts::Command),
}
