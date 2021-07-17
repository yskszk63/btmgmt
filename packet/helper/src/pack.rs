use std::io;

mod imp;
pub use btmgmt_packet_macros::{Pack, Unpack};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no data available.")]
    NoDataAvailable,

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("unexpected value {0}")]
    UnexpectedValue(String),
}

pub type Result<R> = std::result::Result<R, Error>;

pub trait Pack {
    fn pack<W>(&self, write: &mut W) -> Result<()> where W: io::Write;
}

pub trait Unpack: Sized {
    fn unpack<R>(read: &mut R) -> Result<Self> where R: io::Read;
}
