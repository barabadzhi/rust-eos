use hex;

use keys::error as KeyError;

use crate::{ParseAssetError, ParseNameError, ParseSymbolError, ReadError, WriteError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    BytesReadError(ReadError),
    BytesWriteError(WriteError),
    FromHexError(hex::FromHexError),
    Keys(KeyError::Error),
    ParseAssetErr(ParseAssetError),
    ParseNameErr(ParseNameError),
    ParseSymbolError(ParseSymbolError),
    FromTrxKindsError,
    IncreMerkleError,
    InvalidLength,
}

