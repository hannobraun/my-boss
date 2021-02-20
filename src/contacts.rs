use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::Read,
    path::Path,
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Date;

/// Collection of all contacts
#[derive(Debug)]
pub struct Contacts(Vec<Contact>);

impl Contacts {
    /// Load all contacts in a directory
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut contacts = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let contact = Contact::load(entry.path())?;
            contacts.push(contact);
        }

        Ok(Self(contacts))
    }

    /// Iterate over all contacts
    pub fn all(&self) -> impl Iterator<Item = &Contact> {
        self.0.iter()
    }

    /// Iterate over contacts for whom the next communication is due
    pub fn due(&self, date: Date) -> impl Iterator<Item = &Contact> {
        self.0.iter().filter(move |contact| {
            match contact.communication.next_planned() {
                Some(next_planned) => next_planned <= date,
                None => false,
            }
        })
    }
}

/// A contact
#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    /// The contact's name
    pub name: String,

    /// Notes about the contact
    pub notes: Vec<String>,

    /// The organizations that the contact is part of
    pub organizations: Vec<Organization>,

    /// Means of communication with the contact
    ///
    /// The key is a means of communication, like "email" or "phone".
    pub addresses: IndexMap<String, Vec<Address>>,

    /// Links to the contact's website and other online profiles
    pub links: Vec<Link>,

    /// Records of communication with the contact
    pub communication: Communication,
}

impl Contact {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut contact = Vec::new();
        File::open(path)?.read_to_end(&mut contact)?;

        let contact = toml::from_slice(&contact)?;

        Ok(contact)
    }

    pub fn summary(&self) -> impl Display {
        let latest = &self.communication.latest;
        format!(
            "{} (communication to {}; from: {})",
            self.name, latest.to, latest.from,
        )
    }
}

/// An organization that a contact is part of
#[derive(Debug, Deserialize, Serialize)]
pub struct Organization {
    /// The name of the organization
    pub name: String,

    /// The contact's role in the organization
    pub role: Option<String>,
}

/// An address (or equivalent concept) related to a means of communication
///
/// Could be an email address, street address, phone number, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    /// The address itself
    pub value: String,

    /// Notes on this address
    pub notes: Vec<String>,
}

/// A link to a contact's website or other online profile
#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    /// The link itself
    pub value: String,

    /// Notes on this link
    pub notes: Vec<String>,
}

/// The dates of interaction last and next with the contact
#[derive(Debug, Deserialize, Serialize)]
pub struct Communication {
    /// The latest interaction with the contact
    pub latest: LatestCommunication,

    pub planned: Vec<PlannedCommunication>,
}

impl Communication {
    pub fn next_planned(&self) -> Option<Date> {
        let mut next_planned = None;

        for planned in &self.planned {
            let date = next_planned.unwrap_or(planned.date);
            if planned.date <= date {
                next_planned = Some(planned.date);
            }
        }

        next_planned
    }
}

/// The latest communication with a contact
#[derive(Debug, Deserialize, Serialize)]
pub struct LatestCommunication {
    /// The last communication to the contact
    pub to: Date,

    /// The latest communication from the contact
    pub from: Date,
}

/// A planned communication with a contact
#[derive(Debug, Deserialize, Serialize)]
pub struct PlannedCommunication {
    /// The date of the planned communication
    pub date: Date,

    /// Notes about the planned communication
    pub notes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::Contact;

    #[test]
    fn contact_should_match_example_contact_file() -> Result<(), Box<dyn Error>>
    {
        let contact = Contact::load("contacts/ex-ample.toml")?;
        println!("{:#?}", contact);

        // Nothing to check, I think. It's enough that the previous calls don't
        // cause an error.

        Ok(())
    }
}
