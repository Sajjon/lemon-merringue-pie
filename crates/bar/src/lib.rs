mod models;

pub mod prelude {
    pub use crate::models::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("bar");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar() {
        assert_eq!(Bar { bar: true }, Bar { bar: true });
        assert_ne!(Bar { bar: true }, Bar { bar: false });
    }
}
