mod models;

pub mod prelude {
    pub use crate::models::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("farm");
