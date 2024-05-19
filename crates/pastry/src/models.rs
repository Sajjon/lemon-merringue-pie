use crate::prelude::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Pastry {
    pub flour: farm::Flour,
    pub butter: farm::Butter,
    pub eggs: farm::Eggs,
    pub sugar: farm::Sugar,
}

impl Pastry {
    fn new(flour: farm::Flour, butter: farm::Butter, eggs: farm::Eggs, sugar: farm::Sugar) -> Self {
        Self {
            flour,
            butter,
            eggs,
            sugar,
        }
    }

    pub fn bake(
        flour_package: FlourPackage,
        butter_block: ButterBlock,
        egg_box: kitchen::EggBox,
        sugar_package: kitchen::SugarPackage,
    ) -> Self {
        Self::new(
            flour_package.into(),
            butter_block.into(),
            egg_box.into(),
            sugar_package.into(),
        )
    }

    pub fn from_kitchen(kitchen: &Kitchen) -> Self {
        Self::bake(
            kitchen.pantry.flour.clone(),
            kitchen.fridge.butter.clone(),
            kitchen.fridge.egg.clone(),
            kitchen.pantry.sugar.clone(),
        )
    }
}

#[uniffi::export]
pub fn new_pastry_from(kitchen: &kitchen::Kitchen) -> Pastry {
    Pastry::from_kitchen(kitchen)
}
