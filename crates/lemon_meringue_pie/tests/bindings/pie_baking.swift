// import farm
import lemon_meringue_pie

func test() {
  /*
  #[test]
    fn bake() {
        let farm = Farm::new();
        let kitchen = Kitchen::stock_groceries_from_farm(farm.clone());
        let lemon_filling = LemonFilling::from_kitchen(kitchen.clone());
        let meringue = Meringue::from_kitchen(kitchen.clone());
        let pastry = Pastry::from_kitchen(kitchen.clone());
        let pie = LemonMeringuePie::bake(chef, lemon_filling, meringue, pastry);
        let _ = farm.buy(pie);
    }
 */
  let chef = Chef(name: "Auguste Gusteau")
  let farm = Farm(money: Money(amount: 0))
  let produce = farmProduce(farm: farm)
  let kitchen = kitchenStockWith(produce: produce)
  let pie = bakeLemonMeringuePie(chef: chef, kitchen: kitchen)
  let _ = sellPieToFarm(pie: pie, farm: farm)
  // let kitchen = Kitchen.stockGroceriesFromFarm(farm: farm)
  // let lemonFilling = LemonFilling.fromKitchen(kitchen: kitchen)
  // let meringue = Meringue.fromKitchen(kitchen: kitchen)
  // let pastry = Pastry.fromKitchen(kitchen: kitchen)
  // let pie = LemonMeringuePie.bake(
  //   chef: chef,
  //   lemonFilling: lemonFilling,
  //   meringue: meringue,
  //   pastry: pastry
  // )
  // let money = farm.buy(pie)
  // assert(1 == 2)
}

test()
