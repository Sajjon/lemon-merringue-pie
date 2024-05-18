mod models;

pub mod prelude {
    pub use crate::models::*;
}

pub use prelude::*;

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
