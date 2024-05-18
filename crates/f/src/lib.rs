mod models;

pub mod prelude {
    pub use crate::models::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("f");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftypes() {
        assert_eq!(FType { val: true }, FType { val: true });
        assert_eq!(FType2 { val: true }, FType2 { val: true });
    }
}
