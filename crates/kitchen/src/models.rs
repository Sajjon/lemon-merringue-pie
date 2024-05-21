use crate::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Fridge {
    pub egg: EggBox,
    pub butter: ButterBlock,
    pub lemons: Lemons,
    pub firmware_hash: BagOfBytes,
    pub uuid: ApplianceID,
}
impl Fridge {
    pub fn stock_with(produce: farm::Produce) -> Self {
        Self {
            egg: EggBox::from(produce.clone()),
            butter: ButterBlock::from(produce.clone()),
            lemons: Lemons::from(produce.clone()),
            firmware_hash: BagOfBytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            uuid: ApplianceID::default(),
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Pantry {
    pub flour: FlourPackage,
    pub sugar: SugarPackage,
}

impl Pantry {
    pub fn stock_with(produce: farm::Produce) -> Self {
        Self {
            flour: FlourPackage::from(produce.clone()),
            sugar: SugarPackage::from(produce.clone()),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Kitchen {
    pub pantry: Pantry,
    pub fridge: Fridge,
}

#[uniffi::export]
pub fn kitchen_stock_with(produce: farm::Produce) -> Kitchen {
    Kitchen {
        pantry: Pantry::stock_with(produce.clone()),
        fridge: Fridge::stock_with(produce.clone()),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum EggBox {
    Six,
    Twelve,
    TwentyFour,
}
impl Default for EggBox {
    fn default() -> Self {
        Self::Twelve
    }
}
impl From<farm::Eggs> for EggBox {
    fn from(_eggs: Eggs) -> Self {
        Self::default()
    }
}
impl From<EggBox> for farm::Eggs {
    fn from(_eggs: EggBox) -> Self {
        Self
    }
}
impl From<farm::Produce> for EggBox {
    fn from(produce: farm::Produce) -> Self {
        Self::from(produce.eggs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct ButterBlock {
    /// In grams
    pub weight: u16,
}
impl Default for ButterBlock {
    fn default() -> Self {
        Self { weight: 500 }
    }
}
impl From<farm::Butter> for ButterBlock {
    fn from(_butter: Butter) -> Self {
        Self::default()
    }
}

impl From<ButterBlock> for farm::Butter {
    fn from(_butter_block: ButterBlock) -> Self {
        Self
    }
}

impl From<farm::Produce> for ButterBlock {
    fn from(produce: farm::Produce) -> Self {
        Self::from(produce.butter)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct FlourPackage {
    /// In grams
    pub weight: u16,
}
impl Default for FlourPackage {
    fn default() -> Self {
        Self { weight: 2000 }
    }
}
impl From<farm::Flour> for FlourPackage {
    fn from(_flour: Flour) -> Self {
        Self::default()
    }
}
impl From<FlourPackage> for farm::Flour {
    fn from(_flour_package: FlourPackage) -> Self {
        Self
    }
}
impl From<farm::Produce> for FlourPackage {
    fn from(produce: farm::Produce) -> Self {
        Self::from(produce.flour)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct SugarPackage {
    /// In grams
    pub weight: u16,
}
impl Default for SugarPackage {
    fn default() -> Self {
        Self { weight: 1000 }
    }
}

impl From<farm::Sugar> for SugarPackage {
    fn from(_sugar: Sugar) -> Self {
        Self::default()
    }
}

impl From<SugarPackage> for farm::Sugar {
    fn from(_sugar: SugarPackage) -> Self {
        Self
    }
}

impl From<farm::Produce> for SugarPackage {
    fn from(produce: farm::Produce) -> Self {
        Self::from(produce.sugar)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Lemons {
    /// Number of lemons
    pub count: u8,
}
impl Default for Lemons {
    fn default() -> Self {
        Self { count: 3 }
    }
}
impl From<farm::Lemon> for Lemons {
    fn from(_lemon: Lemon) -> Self {
        Self::default()
    }
}

impl From<Lemons> for farm::Lemon {
    fn from(_lemon: Lemons) -> Self {
        Self
    }
}

impl From<farm::Produce> for Lemons {
    fn from(produce: farm::Produce) -> Self {
        Self::from(produce.lemon)
    }
}
