mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use bar::prelude::*;
    pub(crate) use foo::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("foobar");

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(
            Foobar {
                foo: Foo { foo: true },
                foo2: Foo2 { foo: true },
                bar: Bar { bar: true },
                bar2: Bar2 { bar: true },
            },
            Foobar {
                foo: Foo { foo: true },
                foo2: Foo2 { foo: true },
                bar: Bar { bar: true },
                bar2: Bar2 { bar: true },
            }
        );
    }
}
