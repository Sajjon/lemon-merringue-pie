mod models;

pub mod prelude {
    pub use crate::models::*;
    pub use kitchen::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("pastry");
