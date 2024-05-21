use itertools::*;
use std::ops::{AddAssign, SubAssign};

#[derive(uniffi::Record, Default, Clone, PartialEq, Eq, Copy, Debug)]
pub struct Money {
    pub amount: u64,
}

impl From<u64> for Money {
    fn from(value: u64) -> Self {
        Self { amount: value }
    }
}

impl AddAssign<u64> for Money {
    fn add_assign(&mut self, rhs: u64) {
        self.amount.add_assign(rhs)
    }
}

impl AddAssign<Money> for Money {
    fn add_assign(&mut self, rhs: Money) {
        self.add_assign(rhs.amount)
    }
}

impl SubAssign<u64> for Money {
    fn sub_assign(&mut self, rhs: u64) {
        self.amount.sub_assign(rhs)
    }
}

impl SubAssign<Money> for Money {
    fn sub_assign(&mut self, rhs: Money) {
        self.sub_assign(rhs.amount)
    }
}