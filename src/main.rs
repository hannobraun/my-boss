use std::error::Error;

use contacts::Contact;

pub mod contacts;

fn main() -> Result<(), Box<dyn Error>> {
    let contact = Contact::load("contacts/ex-ample.toml")?;
    println!("{:#?}", contact);
    Ok(())
}
