use std::{
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use serde::de::DeserializeOwned;

pub struct TomlFile {
    path: PathBuf,
    buf: Vec<u8>,
}

impl TomlFile {
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let mut buf = Vec::new();

        File::open(path)
            .with_context(|| {
                format!("Failed to open file `{}`", path.display())
            })?
            .read_to_end(&mut buf)
            .with_context(|| {
                format!("Failed to read file `{}`", path.display())
            })?;

        Ok(Self {
            path: path.to_path_buf(),
            buf,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn deserialize<T>(&self) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let value = toml::from_slice(&self.buf).with_context(|| {
            format!("Failed to deserialize contact `{}`", self.path.display())
        })?;

        Ok(value)
    }
}

pub trait TomlValueExt {
    /// Remove empty arrays and tables
    fn normalize(&mut self);
}

impl TomlValueExt for toml::Value {
    fn normalize(&mut self) {
        if let toml::Value::Table(table) = self {
            let mut to_remove = Vec::new();

            for (key, value) in table.iter_mut() {
                if let toml::Value::Array(array) = value {
                    if array.is_empty() {
                        to_remove.push(key.clone());
                    }
                }
                if let toml::Value::Table(table) = value {
                    if table.is_empty() {
                        to_remove.push(key.clone());
                    }

                    // TASK: Step into table.
                }
            }

            for key in to_remove {
                table.remove(&key);
            }
        }
    }
}
