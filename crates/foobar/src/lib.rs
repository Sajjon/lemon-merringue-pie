mod models;

pub use models::*;

uniffi::include_scaffolding!("foobar");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(
            Foobar {
                foo: true,
                bar: false
            },
            Foobar {
                foo: true,
                bar: false
            }
        );
    }
}
