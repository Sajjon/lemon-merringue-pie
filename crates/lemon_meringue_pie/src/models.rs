use std::sync::Arc;

use crate::prelude::*;

// #[derive(Clone, Debug, PartialEq, Eq, Default)]
// pub struct PieID(Uuid);

// uniffi::custom_type!(PieID, Uuid, {
//     remote,
//     from_custom: |pie_id| pie_id.0,
//     try_into_custom: |uuid| Ok(PieID(uuid))
// });

// #[uniffi::export]
// pub fn new_pie_id_default() -> PieID {
//     ApplianceID::default()
// }

/// Lemon Meringue Pie is the most tasty of pies, [here is a recipe].
///
/// It consists of:
/// Meringue (eggs, sugar)
/// Lemon filling (eggs, lemons, flour)
/// Pastry (eggs, flour, butter, sugar)
///
/// [recipe]: https://www.bbc.co.uk/food/recipes/marys_lemon_meringue_pie_02330
#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct LemonMeringuePie {
    pub lemon_filling: LemonFilling,
    pub meringue: Meringue,
    pub pastry: Pastry,
}

impl LemonMeringuePie {
    pub fn bake(
        _chef: Arc<Chef>,
        lemon_filling: LemonFilling,
        meringue: Meringue,
        pastry: Pastry,
    ) -> Self {
        Self {
            lemon_filling,
            meringue,
            pastry,
        }
    }
}

#[uniffi::export]
pub fn bake_lemon_meringue_pie(chef: Arc<Chef>, kitchen: Kitchen) -> LemonMeringuePie {
    LemonMeringuePie::bake(
        chef,
        LemonFilling::from_kitchen(&kitchen),
        Meringue::from_kitchen(&kitchen),
        Pastry::from_kitchen(&kitchen),
    )
}

#[uniffi::export]
pub fn chef_sell_pie_to_farm(chef: Arc<Chef>, _pie: LemonMeringuePie, farm: Arc<Farm>, price: u64) {
    let money = farm.withdraw_money(price);
    chef.deposit_money(money)
}
