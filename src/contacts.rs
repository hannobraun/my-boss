use std::{
    cmp::Ordering,
    fmt::{self, Write as _},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Date;
use walkdir::WalkDir;

use crate::util::toml::load;

/// Collection of all contacts
#[derive(Debug)]
pub struct Contacts(Vec<Contact>);

impl Contacts {
    /// Load all contacts in a directory
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let mut contacts = Vec::new();

        for entry in WalkDir::new(path) {
            let entry = entry?;

            if entry.file_type().is_dir() {
                continue;
            }

            let contact = Contact::load(entry.path()).with_context(|| {
                format!(
                    "Failed to load contact from `{}`",
                    entry.path().display()
                )
            })?;
            contacts.push(contact);
        }

        contacts.sort_by(|a, b| {
            let a_next_planned = a.next_planned_communication();
            let b_next_planned = b.next_planned_communication();

            match (a_next_planned, b_next_planned) {
                (Some(a), Some(b)) => a.cmp(&b),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => a.name.cmp(&b.name),
            }
        });

        Ok(Self(contacts))
    }

    /// Iterate over all contacts
    pub fn all(&self) -> impl Iterator<Item = &Contact> + '_ {
        self.0.iter()
    }

    /// Iterate over contacts for whom the next communication is due
    pub fn due(&self, date: Date) -> impl Iterator<Item = &Contact> + '_ {
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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Contact {
    /// The contact's name
    pub name: String,

    /// Notes about the contact
    pub notes: Vec<String>,

    /// The organizations that the contact is part of
    pub organizations: Vec<Organization>,

    /// Links to the contact's website and other online profiles
    pub links: Vec<Link>,

    /// Means of communication with the contact
    ///
    /// The key is a means of communication, like "email" or "phone".
    pub addresses: IndexMap<String, Vec<Address>>,

    /// Records of communication with the contact
    pub communication: Option<Communication>,
}

impl Contact {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        load(path)
    }

    pub fn create(
        name: String,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<PathBuf> {
        let mut contact = Contact::example()?;
        contact.name = name;

        let name = contact.name.to_lowercase();
        let name = name.replace(" ", "-");
        let name = format!("{}.toml", name);

        let contact = toml::to_vec(&contact)?;

        let path = path.as_ref().join(name).to_path_buf();

        let mut file = File::create(&path)?;
        file.write_all(&contact)?;

        Ok(path)
    }

    pub fn example() -> anyhow::Result<Self> {
        let example = include_bytes!("../contacts/ex-ample.toml");
        let example = toml::from_slice(example)?;
        Ok(example)
    }

    pub fn summary(&self) -> anyhow::Result<impl fmt::Display> {
        let mut summary = String::new();

        write!(summary, "{}", self.name)?;
        if let Some(communication) = &self.communication {
            let mut wrote_something = false;

            if let Some(latest) = &communication.latest {
                if let Some(to) = latest.to {
                    write!(summary, " (latest communication: {}", to)?;
                    wrote_something = true;
                }
            }
            if let Some(planned) = communication.next_planned() {
                if wrote_something {
                    write!(summary, "; next planned: ")?;
                } else {
                    write!(summary, " (next planned communication: ")?;
                    wrote_something = true;
                }

                write!(summary, "{}", planned)?;
            }

            if wrote_something {
                write!(summary, ")")?;
            }
        }

        Ok(summary)
    }

    pub fn next_planned_communication(&self) -> Option<Date> {
        let communication = self.communication.as_ref()?;
        communication.next_planned()
    }
}

/// A link to a contact's website or other online profile
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    /// The link itself
    pub value: String,

    /// Notes on this link
    pub notes: Vec<String>,
}

/// An organization that a contact is part of
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Organization {
    /// The name of the organization
    pub name: String,

    /// The contact's role in the organization
    pub role: Option<String>,
}

/// An address (or equivalent concept) related to a means of communication
///
/// Could be an email address, street address, phone number, etc.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Address {
    /// The address itself
    pub value: String,

    /// Notes on this address
    pub notes: Vec<String>,
}

/// The dates of interaction last and next with the contact
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Communication {
    /// The latest communication with the contact
    pub latest: Option<LatestCommunication>,

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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LatestCommunication {
    /// The last communication to the contact
    pub to: Option<Date>,

    /// The latest communication from the contact
    pub from: Option<Date>,
}

/// A planned communication with a contact
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlannedCommunication {
    /// The date of the planned communication
    pub date: Date,

    /// Notes about the planned communication
    #[serde(default)]
    pub notes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::Contacts;

    #[test]
    fn contacts_should_match_example_files() {
        Contacts::load("contacts").unwrap();

        // Nothing to check, I think. It's enough that the previous call doesn't
        // cause an error.
    }
}
