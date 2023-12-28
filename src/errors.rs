use thiserror::Error;

//? Re-add Error Handling

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to complete network request for '{0}'")]
    ImageRequest(String),
}
