use std::path::PathBuf;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::executables;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::executables;

#[cfg(all(feature = "unique-executables", feature = "rayon"))]
compile_error!("unique-executables and rayon are not compatible");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Executable {
    pub name: String,
    pub path: PathBuf,
}
