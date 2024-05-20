use std::sync::Arc;

use crate::prelude::*;

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
