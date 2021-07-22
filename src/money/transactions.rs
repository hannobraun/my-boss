use std::slice;

use serde::{Deserialize, Serialize};
use time::Date;

use crate::money::amount::Amount;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    /// The date the transaction occurred
    pub date: Date,

    /// Description of the transaction
    pub description: String,

    /// The total amount of the transaction
    pub amount: Amount,
}
