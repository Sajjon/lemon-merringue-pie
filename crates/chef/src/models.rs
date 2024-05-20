use std::sync::RwLock;

use crate::prelude::*;
use std::ops::AddAssign;

#[derive(Default, Debug, uniffi::Object)]
pub struct Chef {
    pub name: String,
    pub money: RwLock<Money>,
}

#[uniffi::export]
impl Chef {
    #[uniffi::constructor]
    pub fn new(name: String, money: Money) -> Arc<Self> {
        Arc::new(Self {
            name,
            money: RwLock::new(money),
        })
    }

    pub fn balance(self: Arc<Self>) -> u64 {
        self.money.try_read().unwrap().amount
    }

    pub fn deposit_money(self: Arc<Self>, money: Money) {
        self.money.try_write().unwrap().add_assign(money)
    }
}
