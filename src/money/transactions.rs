use std::{fmt, path::Path, slice};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::util::toml;

pub struct Transactions<'r>(pub &'r [Transaction]);

impl<'r> IntoIterator for &'r Transactions<'r> {
    type Item = &'r Transaction;
    type IntoIter = slice::Iter<'r, Transaction>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    /// The date the transaction occurred
    pub date: Date,

    /// Description of the transaction
    pub description: String,

    /// The accounts the transaction affected
    pub accounts: Accounts,

    /// The budgets the transaction affected
    pub budgets: Accounts,
}

impl Transaction {
    /// Load a transaction
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        toml::load(path)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Accounts(IndexMap<String, Amount>);

impl Accounts {
    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    pub fn amount_for(&self, name: impl AsRef<str>) -> Option<Amount> {
        self.0.get(name.as_ref()).copied()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Amount(i64);

impl Amount {
    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}€", self.0 / 100, self.0.abs() % 100)
    }
}
