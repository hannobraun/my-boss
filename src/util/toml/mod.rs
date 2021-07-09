pub mod empty_values;
pub mod invalid_keys;

use std::{
    fmt::Write as _,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context as _};
use log::debug;
use serde::{de::DeserializeOwned, Serialize};

/// Validates the provided value against the file
///
/// Makes sure that the provided file doesn't have keys not used by the value.
pub fn validate<T>(value: &T, file: &TomlFile) -> anyhow::Result<()>
where
    T: Serialize,
{
    let buf =
        toml::to_vec(value).context("Failed to re-serialize for validation")?;

    let mut original: toml::Value = file
        .deserialize()
        .context("Failed to deserialize for validation")?;

    let mut roundtrip: toml::Value = toml::from_slice(&buf)
        .context("Failed to roundtrip-deserialize for validation")?;

    original.normalize();
    roundtrip.normalize();

    if original != roundtrip {
        debug!(
            "Failed validation.\n\t Original: {:?}\n\tRoundtrip: {:?}",
            original, roundtrip
        );

        let invalid = original.find_invalid(&roundtrip)?;

        let mut error = String::from("Invalid keys:");

        let mut first_error = true;
        for key in invalid {
            if !first_error {
                write!(error, ",")?;
            }
            first_error = false;

            write!(error, " {}", key)?;
        }

        bail!(error);
    }

    Ok(())
}

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
            format!("Failed to deserialize `{}`", self.path.display())
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
        empty_values::remove(self);
    }

    fn find_invalid(&self, other: &Self) -> anyhow::Result<Vec<String>> {
        let mut invalid = Vec::new();
        invalid_keys::check_value(self, other, &mut invalid, String::new());
        Ok(invalid)
    }
}
