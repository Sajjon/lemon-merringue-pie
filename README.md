# Lemon Meringue Pie

Apart from being [a delicious pie][pie], Lemon Meringue Pie is small demo project of [UniFFI][uf] with Cargo Workspace setup - many creates all using UniFFI that have inter-dependencies on each other.

> [!NOTE]
> In its current form, note that this project does NOT make use of any `ExternalExport` **contrary** to what the [external type UniFFI guide says we must do][ext]. Maybe the guide is outdated? It's of course a good thing that we don't need to state external types in the .udl files, but surprising it works without.

# Cargo tree

Ran `cargo tree --prune uniffi --no-dedupe --package lemon_meringue_pie --format "{lib}"` and then removed "[build-dependencies]" (what is the grep/sed pipe oneliner I'm missing here? :)) yields this tree"

```sh
lemon_meringue_pie
├── chef
│   └── money
├── farm
│   └── money
├── kitchen
│   └── farm
│       └── money
├── lemon_filling
│   ├── farm
│   │   └── money
│   └── kitchen
│       └── farm
│           └── money
├── meringue
│   ├── farm
│   │   └── money
│   └── kitchen
│       └── farm
│           └── money
├── money
└── pastry
    ├── farm
    │   └── money
    └── kitchen
        └── farm
            └── money
```

# Test

## Rust

```rust
#[test]
fn bake() {
    let chef = Chef::new("Auguste Gusteau".to_owned(), Money::from(50));
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
```

## Swift

```swift
func test() {
  let chef = Chef(name: "Auguste Gusteau", money: Money(amount: 50))
  let farm = Farm(money: Money(amount: 200))
  let produce = farm.produce()
  let kitchen = kitchenStockWith(produce: produce)
  let pie = bakeLemonMeringuePie(chef: chef, kitchen: kitchen)

  assert(chef.balance() == 50)
  assert(farm.balance() == 200)

  chefSellPieToFarm(chef: chef, pie: pie, farm: farm, price: 25)

  assert(chef.balance() == 75)
  assert(farm.balance() == 175)
}
```

[uf]: https://github.com/mozilla/uniffi-rs
[ext]: https://mozilla.github.io/uniffi-rs/udl/ext_types_external.html
[pie]: https://www.bbc.co.uk/food/recipes/marys_lemon_meringue_pie_02330
