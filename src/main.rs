use std::error::Error;

use contacts::Contacts;

pub mod contacts;

fn main() -> Result<(), Box<dyn Error>> {
    // TASK: Load path of `contacts/` from a configuration file.
    // TASK: Display only those contacts for whom a planned communication date
    //       is due.
    for contact in Contacts::load("contacts")?.iter() {
        println!("{}", contact.summary());
    }

    Ok(())
}
