use contacts::Contact;

pub mod contacts;

fn main() {
    let contact = Contact::load("contacts/ex-ample.toml");
    println!("{:#?}", contact);
}
