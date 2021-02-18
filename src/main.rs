use std::error::Error;

use contacts::Contacts;

pub mod contacts;

fn main() -> Result<(), Box<dyn Error>> {
    // TASK: Load path of `contacts/` from a configuration file.
    // TASK: Display only those contacts for whom a planned communication date
    //       is due.
    let contacts = Contacts::load("contacts")?;
    // TASK: Display short summary of contact. Name, maybe dates.
    println!("{:#?}", contacts);
    Ok(())
}
