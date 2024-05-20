mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use farm::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("kitchen");

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_etypes() {
//         assert_eq!(EType { val: true }, EType { val: true });
//         assert_eq!(EType2 { val: true }, EType2 { val: true });
//     }
// }
