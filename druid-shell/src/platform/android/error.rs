//! Android Platform Errors

//TODO: add a platform error for macOS, based on NSError

#[derive(Debug, Clone)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Android Error")
    }
}

impl std::error::Error for Error {}
