#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct EType {
    pub val: bool,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct EType2 {
    pub val: bool,
}
