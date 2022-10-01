use std::io;
use std::path;
use toml;

#[derive(Debug, Error)]
pub enum Error {
    Io(io::Error),
    StripPrefixError(path::StripPrefixError),
    TomlError(toml::de::Error),
}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        match self {
            Error::Io(error) => error,
            _ => io::Error::new(io::ErrorKind::Other, "Something happened")
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
