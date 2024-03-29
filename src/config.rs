use std::{
    fs::{File, OpenOptions},
    io::prelude::*,
    path::PathBuf,
};

use anyhow::Context as _;
use serde::{Deserialize, Serialize};

const PATH: &str = "my-boss.toml";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub contacts: PathBuf,
}

impl Config {
    pub fn init() -> anyhow::Result<()> {
        let config = Self::default();
        let config = toml::to_string(&config).with_context(|| {
            format!("Error serializing default configuration ({:?})", config)
        })?;

        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(PATH)
            .with_context(|| {
                format!("Error creating configuration file `{}`", PATH)
            })?
            .write_all(config.as_bytes())
            .with_context(|| {
                format!("Error writing configuration file `{}`", PATH)
            })?;

        Ok(())
    }

    // TASK: Search for configuration file in parent directories.
    pub fn load() -> anyhow::Result<Self> {
        let mut config = String::new();
        File::open(PATH)
            .with_context(|| {
                format!("Error opening configuration file `{}`", PATH)
            })?
            .read_to_string(&mut config)
            .with_context(|| {
                format!("Error reading configuration file `{}`", PATH)
            })?;

        let config = toml::from_str(&config).with_context(|| {
            format!("Error parsing configuration file `{}`", PATH)
        })?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            contacts: PathBuf::from("contacts"),
        }
    }
}
