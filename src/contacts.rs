use std::{
    fmt::{Display, Write as _},
    fs,
    path::Path,
};

use anyhow::{bail, Context as _};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::toml::{TomlFile, TomlValueExt as _};

/// Collection of all contacts
#[derive(Debug)]
pub struct Contacts(Vec<Contact>);

impl Contacts {
    /// Load all contacts in a directory
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut contacts = Vec::new();

        let mut directories = Vec::new();
        directories.push(path.as_ref().to_path_buf());

        while let Some(directory) = directories.pop() {
            let entries = fs::read_dir(&directory).with_context(|| {
                format!("Failed to read directory `{}`", directory.display())
            })?;

            for entry in entries {
                let entry = entry.with_context(|| {
                    format!("Failed to retrieve directory entry")
                })?;

                if entry.path().is_dir() {
                    directories.push(entry.path().to_path_buf());
                    continue;
                }

                let contact =
                    Contact::load(entry.path()).with_context(|| {
                        format!(
                            "Failed to load contact from `{}`",
                            entry.path().display()
                        )
                    })?;
                contacts.push(contact);
            }
        }

        Ok(Self(contacts))
    }

    /// Iterate over all contacts
    pub fn all(&self) -> impl Iterator<Item = &Contact> {
        self.0.iter()
    }

    /// Iterate over contacts for whom the next communication is due
    pub fn due(&self, date: Date) -> impl Iterator<Item = &Contact> {
        // TASK: Sort contacts by next planned date.

        self.0.iter().filter(move |contact| {
            let communication = match &contact.communication {
                Some(communication) => communication,
                None => return false,
            };

            match communication.next_planned() {
                Some(next_planned) => next_planned <= date,
                None => false,
            }
        })
    }
}

/// A contact
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Contact {
    /// The contact's name
    pub name: String,

    /// Notes about the contact
    pub notes: Vec<String>,

    /// Links to the contact's website and other online profiles
    pub links: Vec<Link>,

    /// The organizations that the contact is part of
    pub organizations: Vec<Organization>,

    /// Means of communication with the contact
    ///
    /// The key is a means of communication, like "email" or "phone".
    pub addresses: IndexMap<String, Vec<Address>>,

    /// Records of communication with the contact
    pub communication: Option<Communication>,
}

impl Contact {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file = TomlFile::open(path)?;
        let contact: Self = file.deserialize()?;

        contact.validate(&file)?;

        Ok(contact)
    }

    // TASK: Document.
    pub fn validate(&self, file: &TomlFile) -> anyhow::Result<()> {
        let buf = toml::to_vec(self).with_context(|| {
            format!(
                "Failed re-serialize `{}` for validation",
                file.path().display()
            )
        })?;

        let mut original: toml::Value =
            file.deserialize().with_context(|| {
                format!(
                    "Failed to deserialize `{}` for validation",
                    file.path().display()
                )
            })?;
        let mut roundtrip: toml::Value =
            toml::from_slice(&buf).with_context(|| {
                format!(
                    "Failed to roundtrip-deserialize `{}` for validation",
                    file.path().display()
                )
            })?;

        original.normalize();
        roundtrip.normalize();

        if original != roundtrip {
            if let (
                toml::Value::Table(original),
                toml::Value::Table(roundtrip),
            ) = (&original, &roundtrip)
            {
                let mut error = String::from("Invalid keys:");

                let mut first_error = true;
                for key in original.keys() {
                    if !roundtrip.contains_key(key) {
                        if !first_error {
                            write!(error, ",")?;
                        }
                        first_error = false;

                        write!(error, " {}", key)?;
                    }
                }

                bail!(error);
            }

            bail!(
                "Expected TOML values to be tables: {:?}, {:?}",
                original,
                roundtrip
            );
        }

        Ok(())
    }

    pub fn summary(&self) -> anyhow::Result<impl Display> {
        let mut summary = String::new();

        write!(summary, "{}", self.name)?;
        if let Some(communication) = &self.communication {
            let latest = &communication.latest;
            write!(
                summary,
                " (communication to {}; from: {})",
                latest.to, latest.from,
            )?;
        }

        Ok(summary)
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
    use super::Contacts;

    #[test]
    fn contacts_should_match_example_contact_file() {
        Contacts::load("contacts").unwrap();

        // Nothing to check, I think. It's enough that the previous calls don't
        // cause an error.
    }
}
