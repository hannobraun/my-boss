use std::{fmt, ops, path::Path, slice};

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
}

impl Transaction {
    /// Load a transaction
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        toml::load(path)
    }
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
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

impl From<i64> for Amount {
    fn from(inner: i64) -> Self {
        Self(inner)
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

impl ops::Div for Amount {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 as f64 / rhs.0 as f64
    }
}

impl ops::Div<f64> for Amount {
    type Output = Amount;

    fn div(self, rhs: f64) -> Self::Output {
        Self((self.0 as f64 / rhs).round() as i64)
    }
}

impl ops::Mul<f64> for Amount {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self((self.0 as f64 * rhs).round() as i64)
    }
}

impl ops::Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl ops::Sub for Amount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl ops::SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
