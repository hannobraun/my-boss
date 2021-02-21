use std::{fs::File, io::prelude::*, path::PathBuf};

use anyhow::Context as _;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub contacts: PathBuf,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let path = "my-boss.toml";

        let mut config = Vec::new();
        File::open(path)
            .with_context(|| {
                format!("Error opening configuration file `{}`", path)
            })?
            .read_to_end(&mut config)
            .with_context(|| {
                format!("Error reading configuration file `{}`", path)
            })?;

        let config = toml::from_slice(&config).with_context(|| {
            format!("Error parsing configuration file `{}`", path)
        })?;

        Ok(config)
    }
}
