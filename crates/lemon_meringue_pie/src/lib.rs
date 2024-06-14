mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use chef::prelude::*;
    pub(crate) use farm::prelude::*;
    pub(crate) use lemon_filling::prelude::*;
    pub(crate) use meringue::prelude::*;

    pub(crate) use pastry::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("lemon_meringue_pie");

#[cfg(test)]
mod tests {
    use super::*;
    use money::prelude::*;

    #[test]
    fn bake() {
        let chef = Chef::new("Auguste Gusteau".to_owned(), Money::from(50)).unwrap();
        let farm = Farm::new(Money::from(200));
        let produce = farm.clone().produce();
        let kitchen = kitchen_stock_with(produce);
        let pie = bake_lemon_meringue_pie(chef.clone(), kitchen);

        assert_eq!(chef.clone().balance(), 50);
        assert_eq!(farm.clone().balance(), 200);

        chef_sell_pie_to_farm(chef.clone(), pie, farm.clone(), 25);

        assert_eq!(chef.clone().balance(), 75);
        assert_eq!(farm.clone().balance(), 175);
    }
}
