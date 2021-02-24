use std::error::Error;

use clap::Clap as _;
use my_boss::{
    args::{Args, Command},
    config::Config,
    contacts::Contacts,
};
use time::OffsetDateTime;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            Config::init()?;
        }
        Command::Contacts => {
            // TASK: Add flag that prints all contacts.

            let config = Config::load()?;

            // TASK: Use local time instead. As of this writing, this is blocked
            //       by this issue: https://github.com/time-rs/time/issues/293
            let today = OffsetDateTime::now_utc().date();
            for contact in Contacts::load(config.contacts)?.due(today) {
                println!("{}", contact.summary()?);
            }
        }
    }

    Ok(())
}
