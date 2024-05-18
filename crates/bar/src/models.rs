use crate::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Bar {
    pub e: EType,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct Bar2 {
    pub e: EType2,
}
