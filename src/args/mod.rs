pub mod contacts;

#[derive(clap::Parser, Clone)]
#[clap(name = "My Boss")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Parser, Clone)]
pub enum Command {
    /// Create a configuration file with default values
    Init,

    /// Manage contacts
    #[clap(subcommand)]
    Contacts(contacts::Command),
}
