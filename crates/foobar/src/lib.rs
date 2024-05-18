mod models;

pub use models::*;

uniffi::include_scaffolding!("foobar");

#[cfg(test)]
mod tests {
    use bar::Bar;
    use foo::Foo;

    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(
            Foobar {
                foo: Foo { foo: true },
                bar: Bar { bar: true },
            },
            Foobar {
                foo: Foo { foo: true },
                bar: Bar { bar: true },
            }
        );
    }
}
