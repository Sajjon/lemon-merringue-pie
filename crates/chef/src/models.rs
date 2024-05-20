use std::sync::Arc;

#[derive(Default, Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct Chef {
    pub name: String,
}

#[uniffi::export]
impl Chef {
    #[uniffi::constructor]
    pub fn new(name: String) -> Arc<Self> {
        Arc::new(Self { name })
    }
}
