use std::error::Error;

use config::Config;
use contacts::Contacts;

pub mod config;
pub mod contacts;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load();
    // TASK: Display only those contacts for whom a planned communication date
    //       is due.
    for contact in Contacts::load(config.contacts)?.iter() {
        println!("{}", contact.summary());
    }

    Ok(())
}
