/// Custom Result type with two generic parameters for user convenience
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Describes possible errors that might happen when user interacts with this crate
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("nix crate error: {0}")]
    Nix(#[from] nix::Error),
    #[error("etherparse crate value error: {0}")]
    EtherparseValue(#[from] etherparse::ValueError),
    #[error("etherparse crate write error: {0}")]
    EtherparseWrite(#[from] etherparse::WriteError),
}
