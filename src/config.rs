use std::path::PathBuf;

pub struct Config {
    pub contacts: PathBuf,
}

impl Config {
    pub fn load() -> Self {
        // TASK: Actually load configuration.
        Self {
            contacts: PathBuf::from("contacts"),
        }
    }
}
