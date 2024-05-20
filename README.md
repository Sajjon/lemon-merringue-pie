# Lemon Meringue Pie

Apart from being [a delicious pie][pie], Lemon Meringue Pie is small demo project of [UniFFI][uf] with Cargo Workspace setup - many creates all using UniFFI that have inter-dependencies on each other.

> [!NOTE]
> In its current form, note that this project does NOT make use of any `ExternalExport` **contrary** to what the [external type UniFFI guide says we must do][ext]. Maybe the guide is outdated? It's of course a good thing that we don't need to state external types in the .udl files, but surprising it works without.

Cargo tree:
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

[uf]: https://github.com/mozilla/uniffi-rs
[ext]: https://mozilla.github.io/uniffi-rs/udl/ext_types_external.html
[pie]: https://www.bbc.co.uk/food/recipes/marys_lemon_meringue_pie_02330
