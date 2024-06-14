use std::{
    ops::SubAssign,
    sync::{Arc, RwLock},
};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BatchID(Uuid);

uniffi::remote_type!(Uuid, common);

uniffi::custom_type!(BatchID, Uuid, {
    remote,
    from_custom: |batch_id| batch_id.0,
    try_into_custom: |uuid| Ok(BatchID(uuid))
});

#[uniffi::export]
pub fn new_batch_id_random() -> BatchID {
    BatchID(Uuid::new_v4())
}

#[derive(Clone, Debug, uniffi::Record)]
pub struct Produce {
    pub batch: BatchID,
    pub epoch: Epoch,
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
            batch: new_batch_id_random(),
            epoch: Epoch::from(1337),
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
