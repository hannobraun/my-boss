use std::{fmt, ops, path::Path, slice};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::util::toml;

#[derive(Clone, Debug)]
pub struct Transactions(Vec<Transaction>);

impl Transactions {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Self(transactions)
    }

    pub fn total(&self) -> Amount {
        let mut total = Amount::zero();

        for transaction in &self.0 {
            total += transaction.amount;
        }

        total
    }

    pub fn account_total(&self, name: &str) -> Amount {
        let mut total = Amount::zero();

        for transaction in &self.0 {
            if let Some(amount) = transaction.budgets.amount_for(name) {
                total += amount;
            }
        }

        total
    }
}

impl From<Vec<Transaction>> for Transactions {
    fn from(inner: Vec<Transaction>) -> Self {
        Self::new(inner)
    }
}

impl<'r> IntoIterator for &'r Transactions {
    type Item = &'r Transaction;
    type IntoIter = slice::Iter<'r, Transaction>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'r> IntoIterator for &'r mut Transactions {
    type Item = &'r mut Transaction;
    type IntoIter = slice::IterMut<'r, Transaction>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    /// The date the transaction occurred
    pub date: Date,

    /// Description of the transaction
    pub description: String,

    /// The total amount of the transaction
    pub amount: Amount,

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
    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    pub fn insert(&mut self, name: impl Into<String>, amount: Amount) {
        self.0.insert(name.into(), amount);
    }

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
    pub fn from_int_and_frac(int: i64, frac: i64) -> Self {
        Self(int * 100 + frac)
    }

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

impl ops::Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
