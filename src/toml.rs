use std::{
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use log::debug;
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

    fn find_invalid(&self, other: &Self) -> anyhow::Result<Vec<String>>;
}

impl TomlValueExt for toml::Value {
    fn normalize(&mut self) {
        if let toml::Value::Table(table) = self {
            normalize_inner(table);
        }
    }

    fn find_invalid(&self, other: &Self) -> anyhow::Result<Vec<String>> {
        let mut invalid = Vec::new();
        check_value(self, other, &mut invalid);
        Ok(invalid)
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

fn check_value(
    from: &toml::Value,
    to: &toml::Value,
    invalid: &mut Vec<String>,
) {
    debug!("Checking value:\n\t{:?}\n\t{:?}", from, to);

    if let (toml::Value::Table(from), toml::Value::Table(to)) = (from, to) {
        check_table(from, to, invalid);
    }
    if let (toml::Value::Array(from), toml::Value::Array(to)) = (from, to) {
        check_array(from, to, invalid);
    }
}

fn check_table(
    from: &toml::value::Table,
    to: &toml::value::Table,
    invalid: &mut Vec<String>,
) {
    debug!("Checking value:\n\t{:?}\n\t{:?}", from, to);

    for (key, from_value) in from.iter() {
        match to.get(key) {
            Some(to_value) => {
                check_value(from_value, to_value, invalid);
            }
            None => {
                invalid.push(key.clone());
            }
        }
    }
}

fn check_array(
    from: &toml::value::Array,
    to: &toml::value::Array,
    invalid: &mut Vec<String>,
) {
    debug!("Checking value:\n\t{:?}\n\t{:?}", from, to);

    for (from_item, to_item) in from.iter().zip(to.iter()) {
        check_value(from_item, to_item, invalid);
    }
}
