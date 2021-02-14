// TASK: Add code to serialize to and deserialize from TOML.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    pub addresses: HashMap<String, Vec<Address>>,

    /// Records of communication with the contact
    pub communication: Communication,
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

/// The dates of interaction last and next with the contact
#[derive(Debug, Deserialize, Serialize)]
pub struct Communication {
    /// The latest interaction with the contact
    pub latest: LatestCommunication,

    pub planned: Vec<PlannedCommunication>,
}

/// The latest communication with a contact
#[derive(Debug, Deserialize, Serialize)]
pub struct LatestCommunication {
    /// The last communication to the contact
    ///
    /// TASK: Use structure date type.
    pub to: String,

    /// The latest communication from the contact
    ///
    /// TASK: Use structure date type.
    pub from: String,
}

/// A planned communication with a contact
#[derive(Debug, Deserialize, Serialize)]
pub struct PlannedCommunication {
    /// The date of the planned communication
    ///
    /// TASK: Use structure date type.
    pub date: String,

    /// Notes about the planned communication
    pub notes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::Contact;

    #[test]
    fn contact_should_match_example_contact_file() {
        let contact = include_str!("../contacts/ex-ample.toml");
        let contact: Contact = toml::from_str(contact).unwrap();

        println!("{:?}", contact);

        // Nothing to check, I think. It's enough that the previous calls don't
        // panic.
    }
}
