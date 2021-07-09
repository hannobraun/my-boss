pub mod empty_values;
pub mod invalid_keys;

use std::{fmt::Write as _, fs::File, io::prelude::*, path::Path};

use anyhow::{bail, Context as _};
use log::debug;
use serde::{de::DeserializeOwned, Serialize};

pub fn load<T>(path: impl AsRef<Path>) -> anyhow::Result<T>
where
    T: DeserializeOwned + Serialize,
{
    let path = path.as_ref();

    let file = TomlFile::open(path)?;
    let value: T = deserialize(&file.buf, path)?;

    validate(&value, &file, path)?;

    Ok(value)
}

/// Validates the provided value against the file
///
/// Makes sure that the provided file doesn't have keys not used by the value.
fn validate<T>(value: &T, file: &TomlFile, path: &Path) -> anyhow::Result<()>
where
    T: Serialize,
{
    let buf =
        toml::to_vec(value).context("Failed to re-serialize for validation")?;

    let mut original: toml::Value = deserialize(&file.buf, path)
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

fn deserialize<T>(buf: &[u8], path: &Path) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let value = toml::from_slice(buf).with_context(|| {
        format!("Failed to deserialize `{}`", path.display())
    })?;

    Ok(value)
}

pub struct TomlFile {
    buf: Vec<u8>,
}

impl TomlFile {
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        let mut buf = Vec::new();

        File::open(path)
            .with_context(|| {
                format!("Failed to open file `{}`", path.display())
            })?
            .read_to_end(&mut buf)
            .with_context(|| {
                format!("Failed to read file `{}`", path.display())
            })?;

        Ok(Self { buf })
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
