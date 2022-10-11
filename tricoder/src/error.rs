use thiserror::Error; // This library provides a convenient derive macro for the standard library's std::error::Error trait.


#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: tricoder <target.com>")]
    CliUsage,
}