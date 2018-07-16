#![allow(missing_docs)]
#[cfg(all(unix, not(target_os = "macos")))]
use dbus;
use failure;

use std::num;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display="")]
    #[cfg(all(unix, not(target_os = "macos")))]
    Dbus(dbus::Error),

    #[fail(display="")]
    Parse(num::ParseIntError),

    #[fail(display="The running server supplied an unknown version: {}", _0 )]
    SpecVersion(String),

    #[fail(display="ParseError: {}", error)]
    ParseError{ #[cause] error: ::std::num::ParseIntError },

}

impl From<dbus::Error> for Error {
    fn from(e: dbus::Error) -> Error {
        Error::Parse(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        Error::Parse(e)
    }
}
