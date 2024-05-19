mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use common::prelude::*;
    pub(crate) use farm::prelude::*;
    pub(crate) use kitchen::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("lemon_filling");

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bar() {
//         assert_eq!(Bar::default(), Bar::default());
//     }
// }
