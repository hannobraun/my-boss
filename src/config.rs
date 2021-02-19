use std::{error::Error, path::PathBuf};

pub struct Config {
    pub contacts: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        // TASK: Actually load configuration.
        Ok(Self {
            contacts: PathBuf::from("contacts"),
        })
    }
}
