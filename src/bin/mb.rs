use std::error::Error;

use my_boss::{config::Config, contacts::Contacts};
use time::OffsetDateTime;

fn main() -> Result<(), Box<dyn Error>> {
    // TASK: Only display contacts, if a command is passed: `mb contacts`
    // TASK: Add `mb init` command that creates configuration file with default
    //       values.

    let config = Config::load()?;
    // TASK: Use local time instead. As of this writing, this is blocked by this
    //       issue: https://github.com/time-rs/time/issues/293
    let today = OffsetDateTime::now_utc().date();
    for contact in Contacts::load(config.contacts)?.due(today) {
        println!("{}", contact.summary());
    }

    Ok(())
}
