use thiserror::Error as ThisError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(u32)]
#[derive(Debug, ThisError, uniffi::Error)]
pub enum Error {
    #[error("Invalid Epoch string: '{bad_epoch}'")]
    InvalidEpoch { bad_epoch: String } = 100,

    #[error("Chef cannot be broke!!")]
    ChefMustStartWithMoney = 101,
}

impl Error {
    pub fn error_code(&self) -> u32 {
        core::intrinsics::discriminant_value(self)
    }
}

#[uniffi::export]
pub fn error_message_from_error(error: &Error) -> String {
    error.to_string()
}

#[uniffi::export]
pub fn error_code_from_error(error: &Error) -> u32 {
    error.error_code()
}
