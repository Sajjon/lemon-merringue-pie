mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use common::prelude::*;
    pub(crate) use farm::prelude::*;
    pub(crate) use kitchen::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("meringue");

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_otypes() {
//         assert_eq!(OType { val: true }, OType { val: true });
//         assert_eq!(OType2 { val: true }, OType2 { val: true });
//     }
// }
