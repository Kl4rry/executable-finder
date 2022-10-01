use std::path::PathBuf;

#[cfg(unix)]
mod unix;
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
