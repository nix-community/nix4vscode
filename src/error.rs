use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown")]
    Unknown,
    #[error("Fail to query sha256 of {0}")]
    Sha256Error(String),
    #[error("CacheMissing {0}")]
    CacheMissing(String),
    #[error("Attribute {0} missing")]
    AttributeMissing(String),
}
