use std::io;
use std::path;

#[derive(Debug, Error)]
pub enum Error {
		Io(io::Error),
		StripPrefixError(path::StripPrefixError)
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
