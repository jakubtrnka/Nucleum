pub type Result<T> = std::result::Result<T, NucleumError>;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NucleumError {
    #[error("{0}")]
    ConnectionError(String),
}

impl From<tokio::io::Error> for NucleumError {
    fn from(e: tokio::io::Error) -> Self {
        Self::ConnectionError(e.to_string())
    }
}
