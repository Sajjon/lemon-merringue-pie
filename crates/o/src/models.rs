#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct OType {
    pub val: bool,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct OType2 {
    pub val: bool,
}
