use std::error::Error;

use contacts::Contact;

pub mod contacts;

fn main() -> Result<(), Box<dyn Error>> {
    // TASK: Load all contacts from `contacts/`.
    // TASK: Load path of `contacts/` from a configuration file.
    // TASK: Display only those contacts for whom a planned communication date
    //       is due.
    let contact = Contact::load("contacts/ex-ample.toml")?;
    // TASK: Display short summary of contact. Name, maybe dates.
    println!("{:#?}", contact);
    Ok(())
}
