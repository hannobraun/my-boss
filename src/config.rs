use std::{fs::File, io::prelude::*, path::PathBuf};

use anyhow::Context as _;
use serde::{Deserialize, Serialize};

const PATH: &str = "my-boss.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub contacts: PathBuf,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let mut config = Vec::new();
        File::open(PATH)
            .with_context(|| {
                format!("Error opening configuration file `{}`", PATH)
            })?
            .read_to_end(&mut config)
            .with_context(|| {
                format!("Error reading configuration file `{}`", PATH)
            })?;

        let config = toml::from_slice(&config).with_context(|| {
            format!("Error parsing configuration file `{}`", PATH)
        })?;

        Ok(config)
    }
}
