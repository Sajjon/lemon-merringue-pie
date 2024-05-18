mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use e::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("bar");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar() {
        assert_eq!(Bar::default(), Bar::default());
    }
}
