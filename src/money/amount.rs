use std::{fmt, ops};

use serde::{Deserialize, Serialize};

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
