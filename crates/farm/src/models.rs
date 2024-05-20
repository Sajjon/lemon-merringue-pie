use std::{
    ops::SubAssign,
    sync::{Arc, RwLock},
};

use crate::prelude::*;

#[derive(Clone, Debug, uniffi::Record)]
pub struct Produce {
    pub producer: Arc<Farm>,
    pub eggs: Eggs,
    pub butter: Butter,
    pub flour: Flour,
    pub sugar: Sugar,
    pub lemon: Lemon,
}

#[derive(Debug, uniffi::Object)]
pub struct Farm {
    pub money: RwLock<Money>,
}

#[uniffi::export]
impl Farm {
    #[uniffi::constructor]
    pub fn new(money: Money) -> Arc<Self> {
        Arc::new(Self {
            money: RwLock::new(money),
        })
    }

    pub fn balance(self: Arc<Self>) -> u64 {
        self.money.try_read().unwrap().amount
    }

    pub fn withdraw_money(self: Arc<Self>, amount: u64) -> Money {
        self.money.try_write().unwrap().sub_assign(amount);
        Money { amount }
    }

    pub fn produce(self: Arc<Self>) -> Produce {
        Produce {
            producer: self.clone(),
            eggs: Eggs,
            butter: Butter,
            flour: Flour,
            sugar: Sugar,
            lemon: Lemon,
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Eggs;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Butter;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Flour;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Sugar;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Lemon;
