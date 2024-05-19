use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Produce {
    pub producer: Farm,
    pub eggs: Eggs,
    pub butter: Butter,
    pub flour: Flour,
    pub sugar: Sugar,
    pub lemon: Lemon,
}

#[derive(uniffi::Record, Default, Clone, PartialEq, Eq, Copy, Debug)]
pub struct Money {
    pub amount: u64,
}

#[derive(Clone, Default, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Farm {
    pub money: Money,
}

#[uniffi::export]
pub fn new_farm() -> Farm {
    Farm::default()
}

#[uniffi::export]
pub fn farm_produce(farm: &Farm) -> Produce {
    Produce {
        producer: farm.clone(),
        eggs: Eggs::default(),
        butter: Butter::default(),
        flour: Flour::default(),
        sugar: Sugar::default(),
        lemon: Lemon::default(),
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
