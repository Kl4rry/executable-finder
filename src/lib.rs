use std::{env, error, fmt, io, path::PathBuf};

#[cfg(unix)]
pub mod unix;
#[cfg(unix)]
pub use unix::executables;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::executables;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Executable {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum ExeError {
    Io(io::Error),
    Env(env::VarError),
}

impl fmt::Display for ExeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExeError::Io(ref e) => e.fmt(f),
            ExeError::Env(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for ExeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ExeError::Io(ref e) => Some(e),
            ExeError::Env(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for ExeError {
    fn from(err: io::Error) -> ExeError {
        ExeError::Io(err)
    }
}

impl From<env::VarError> for ExeError {
    fn from(err: env::VarError) -> ExeError {
        ExeError::Env(err)
    }
}
