use bar::Bar;
use foo::Foo;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Foobar {
    pub foo: Foo,
    pub bar: Bar,
}
