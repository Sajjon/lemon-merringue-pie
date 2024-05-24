use crate::prelude::*;

// Generate the FfiConverter needed by UniFFI for newtype `Epoch`.
uniffi::custom_newtype!(Epoch, u64);

/// A type-safe consensus epoch number.
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, derive_more::Display, derive_more::Debug,
)]
pub struct Epoch(pub u64);

#[uniffi::export]
pub fn epoch_get_value(epoch: &Epoch) -> u64 {
    epoch.0
}

#[uniffi::export]
pub fn new_epoch_from_str(epoch: String) -> Result<Epoch> {
    epoch
        .parse::<u64>()
        .map(Epoch::from)
        .map_err(|_| Error::InvalidEpoch { bad_epoch: epoch })
}

impl From<u64> for Epoch {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Epoch> for u64 {
    fn from(value: Epoch) -> Self {
        value.0
    }
}
