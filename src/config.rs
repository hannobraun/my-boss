use std::{error::Error, fs::File, io::prelude::*, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub contacts: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let mut config = Vec::new();
        File::open("my-boss.toml")?.read_to_end(&mut config)?;

        let config = toml::from_slice(&config)?;

        Ok(config)
    }
}
