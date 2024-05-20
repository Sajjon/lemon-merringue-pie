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

# Setup Kotlin

```sh
brew install kotlin
```

## Kotlin dependencies

> [!IMPORTANT]  
> To run tests in Kotlin you also need to download
> [JNA](https://mvnrepository.com/artifact/net.java.dev.jna/jna)
>
> ```sh
> curl https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.14.0/jna-5.14.0.jar --output jna-5.14.0.jar
> ```

## `direnv`

Install [`direnv`](https://direnv.net/) in order to automatically load `CLASSPATH` and `JAVA_OPTS` in [`.envrc`](.envrc), so that you can run Kotlin bindgen tests from cli using the command in the bottom of this document - i.e. without having to export `CLASSPATH``.

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

test()
```

## Kotlin

For Kotlin we have to import all packages

```kotlin
import com.sajjon.lemon.meringue.pie.*
import com.sajjon.chef.*
import com.sajjon.money.*
import com.sajjon.farm.*
import com.sajjon.pastry.*
import com.sajjon.meringue.*
import com.sajjon.lemon.filling.*
import com.sajjon.kitchen.*

fun test() {
    val chef = Chef(name = "Auguste Gusteau", money = Money(amount = 50u))
    val farm = Farm(money = Money(amount = 200u))
    val produce = farm.produce()
    val kitchen = kitchenStockWith(produce = produce)
    val pie = bakeLemonMeringuePie(chef = chef, kitchen = kitchen)

    assert(chef.balance() == 50.toULong())
    assert(farm.balance() == 200.toULong())

    chefSellPieToFarm(chef = chef, pie = pie, farm = farm, price = 25u)

    assert(chef.balance() == 75.toULong())
    assert(farm.balance() == 175.toULong())
}

test()
```

[uf]: https://github.com/mozilla/uniffi-rs
[ext]: https://mozilla.github.io/uniffi-rs/udl/ext_types_external.html
[pie]: https://www.bbc.co.uk/food/recipes/marys_lemon_meringue_pie_02330
