mod models;

pub mod prelude {
    pub use crate::models::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("o");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otypes() {
        assert_eq!(OType { val: true }, OType { val: true });
        assert_eq!(OType2 { val: true }, OType2 { val: true });
    }
}
