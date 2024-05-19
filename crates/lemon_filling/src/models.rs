use crate::prelude::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct LemonFilling {
    pub lemon: farm::Lemon,
    pub flour: farm::Flour,
    pub sugar: farm::Sugar,
    pub eggs: farm::Eggs,
}

impl LemonFilling {
    fn new(lemon: farm::Lemon, flour: farm::Flour, sugar: farm::Sugar, eggs: farm::Eggs) -> Self {
        Self {
            lemon,
            flour,
            sugar,
            eggs,
        }
    }
    pub fn bake(
        lemons: kitchen::Lemons,
        flour: kitchen::FlourPackage,
        sugar: kitchen::SugarPackage,
        eggs: kitchen::EggBox,
    ) -> Self {
        Self::new(lemons.into(), flour.into(), sugar.into(), eggs.into())
    }

    pub fn from_kitchen(kitchen: &Kitchen) -> LemonFilling {
        Self::bake(
            kitchen.fridge.lemons.clone(),
            kitchen.pantry.flour.clone(),
            kitchen.pantry.sugar.clone(),
            kitchen.fridge.egg.clone(),
        )
    }
}

#[uniffi::export]
pub fn new_lemon_filling_from(kitchen: &kitchen::Kitchen) -> LemonFilling {
    LemonFilling::from_kitchen(kitchen)
}
