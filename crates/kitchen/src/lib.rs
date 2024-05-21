mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use common::prelude::*;
    pub(crate) use farm::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("kitchen");
