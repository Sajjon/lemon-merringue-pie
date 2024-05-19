mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use chef::prelude::*;
    pub(crate) use common::prelude::*;
    pub(crate) use farm::prelude::*;
    pub(crate) use lemon_filling::prelude::*;
    pub(crate) use meringue::prelude::*;
    pub(crate) use pastry::prelude::*;
}

pub use prelude::*;

// #[uniffi::export(with_foreign)]
// trait Buying: Sync + Send {
//     fn buy(&self, produce: Arc<dyn Produce>) -> Money;
// }

// #[uniffi::export]
// impl Buying for Farm {
//     fn buy(&self, _produce: Arc<dyn Produce>) -> Money {
//         Money
//     }
// }

#[uniffi::export]
pub fn sell_pie_to_farm(_pie: LemonMeringuePie, _farm: &Farm) -> Money {
    Money { amount: 1337 }
}

// impl Produce for LemonMeringuePie {}

uniffi::include_scaffolding!("lemon_meringue_pie");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bake() {
        let chef = Chef::new("Auguste Gusteau".to_owned());
        let farm = new_farm();
        let produce = farm_produce(&farm);
        let kitchen = kitchen_stock_with(produce);
        let pie = bake_lemon_meringue_pie(chef, kitchen);
        let _ = sell_pie_to_farm(pie, &farm);
    }
}
