use crate::prelude::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct EggWhite;
impl From<Eggs> for EggWhite {
    fn from(_eggs: Eggs) -> Self {
        EggWhite
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Meringue {
    pub egg_whites: EggWhite,
    pub sugar: farm::Sugar,
}

impl Meringue {
    fn new(egg_whites: EggWhite, sugar: farm::Sugar) -> Self {
        Self { egg_whites, sugar }
    }

    pub fn bake(egg_box: kitchen::EggBox, sugar: kitchen::SugarPackage) -> Self {
        Self::new(EggWhite::from(Eggs::from(egg_box)), sugar.into())
    }

    pub fn from_kitchen(kitchen: &Kitchen) -> Self {
        Self::bake(kitchen.fridge.egg.clone(), kitchen.pantry.sugar.clone())
    }
}

#[uniffi::export]
pub fn new_meringue_from(kitchen: &kitchen::Kitchen) -> Meringue {
    Meringue::from_kitchen(kitchen)
}
