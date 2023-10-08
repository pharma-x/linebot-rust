use axum::extract::rejection::FormRejection;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    JsonRejection(#[from] axum::extract::rejection::JsonRejection),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

#[derive(Debug, Error)]
pub enum SignatureVerificationError {
    #[error("Cannnot create instanced mac with the channel secret as the key")]
    CannotCreateMac,
    #[error("Unauthorized: Invalid signature")]
    InvalidSignature,
}
