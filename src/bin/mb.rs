use std::error::Error;

use my_boss::{config::Config, contacts::Contacts};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;
    // TASK: Display only those contacts for whom a planned communication date
    //       is due.
    for contact in Contacts::load(config.contacts)?.all() {
        println!("{}", contact.summary());
    }

    Ok(())
}
