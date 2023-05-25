use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to complete network request for '{0}'")]
    ImageRequest(String),

    #[error("Failed to reach the domain for '{0}'")]
    Domain(String),

    #[error("Failed to set image using feh '{0}'")]
    Feh(String),

    #[error("Failed to retrieve save directory")]
    Directory,
}
