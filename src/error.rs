use std::fmt;
use std::io;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    Str(String, Option<Box<StdError + Send + Sync>>),
    Std(Box<StdError + Send + Sync>),
    Io(io::Error),
}

impl Error {
    pub fn context<S: Into<String>>(self, s: S) -> Error {
        let s = s.into();
        match self {
            Error::Str(old, reason) => Error::Str(s, Some(Box::new(Error::Str(old, reason)))),
            Error::Std(err) => Error::Str(s, Some(err)),
            Error::Io(err) => Error::Str(s, Some(Box::new(err))),
        }
    }
}

pub type Res<T> = Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Str(ref desc, ref reason) => {
                write!(f, "{}", desc)?;
                if let Some(ref reason) = *reason {
                    write!(f, "\n  because: {}", reason)?;
                }
                Ok(())
            }
            Error::Std(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Str(ref s, _) => s.as_str(),
            Error::Std(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Str(_, ref err) => err.as_ref().map(|e|e.as_ref() as &StdError),
            Error::Std(ref err) => Some(&**err),
            Error::Io(ref err) => Some(&*err),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        Error::Str(s.to_owned(), None)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Str(s, None)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
