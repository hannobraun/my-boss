use std::{
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use log::{debug, trace};
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

    fn differences_to(&self, other: &Self) -> anyhow::Result<Vec<String>>;
}

impl TomlValueExt for toml::Value {
    fn normalize(&mut self) {
        if let toml::Value::Table(table) = self {
            normalize_inner(table);
        }
    }

    fn differences_to(&self, other: &Self) -> anyhow::Result<Vec<String>> {
        let mut differences = Vec::new();
        differences_to_inner(self, other, &mut differences);
        Ok(differences)
    }
}

fn normalize_inner(table: &mut toml::value::Table) {
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

fn differences_to_inner(
    from: &toml::Value,
    to: &toml::Value,
    differences: &mut Vec<String>,
) {
    debug!("Checking differences:\n\t{:?}\n\t{:?}", from, to);

    let mut to_check = Vec::new();

    if let (toml::Value::Table(self_), toml::Value::Table(other)) = (from, to) {
        to_check.push((self_, other));
    }

    for (self_, other) in to_check {
        for key in self_.keys() {
            trace!("Checking \"{}\"", key);

            if !other.contains_key(key) {
                differences.push(key.clone());
            }
        }
    }
}
