use clap::Clap;

#[derive(Clap)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap)]
pub enum Command {
    Contacts,
}
