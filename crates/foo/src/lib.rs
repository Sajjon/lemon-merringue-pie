mod models;

pub use models::*;

uniffi::include_scaffolding!("foo");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(Foo { foo: true }, Foo { foo: true });
        assert_ne!(Foo { foo: true }, Foo { foo: false });
    }
}
