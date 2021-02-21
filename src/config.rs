use std::{fs::File, io::prelude::*, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub contacts: PathBuf,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let path = "my-boss.toml";

        let mut config = Vec::new();
        File::open(path)?.read_to_end(&mut config)?;

        let config = toml::from_slice(&config)?;

        Ok(config)
    }
}
