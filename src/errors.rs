use std::io;
use std::string::FromUtf8Error;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum RpcError {
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "serde_json error: {}", _0)]
    Serde(#[cause] serde_json::Error),
    // #[fail(display = "Unexpected command type")]
    // UnexpectedCommandType,
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),
    #[fail(display = "{}", _0)]
    StringError(String),
}

impl From<io::Error> for RpcError {
    fn from(err: io::Error) -> RpcError {
        RpcError::Io(err)
    }
}

impl From<serde_json::Error> for RpcError {
    fn from(err: serde_json::Error) -> RpcError {
        RpcError::Serde(err)
    }
}

impl From<FromUtf8Error> for RpcError {
    fn from(err: FromUtf8Error) -> RpcError {
        RpcError::Utf8(err)
    }
}

pub type Result<T> = std::result::Result<T, RpcError>;
