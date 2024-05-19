mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use common::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("farm");

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_etypes() {
//         assert_eq!(EType { val: true }, EType { val: true });
//         assert_eq!(EType2 { val: true }, EType2 { val: true });
//     }
// }
