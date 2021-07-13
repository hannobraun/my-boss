use std::{fmt, ops, path::Path, slice};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::util::toml;

pub struct Transactions<'r>(&'r [Transaction]);

impl<'r> Transactions<'r> {
    pub fn new(transactions: &'r [Transaction]) -> Self {
        Self(transactions)
    }

    pub fn accounts_total(&self, name: &str) -> Amount {
        let mut total = Amount::zero();

        for transaction in self.0 {
            if let Some(amount) = transaction.accounts.amount_for(name) {
                total += amount;
            }
        }

        total
    }

    pub fn budgets_total(&self, name: &str) -> Amount {
        let mut total = Amount::zero();

        for transaction in self.0 {
            if let Some(amount) = transaction.budgets.amount_for(name) {
                total += amount;
            }
        }

        total
    }
}

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
    /// Construct a new amount with the value of zero
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}€", self.0 / 100, self.0.abs() % 100)
    }
}

impl ops::AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
