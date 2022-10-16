#[cfg(feature = "rayon")]
use rayon::prelude::*;

use std::{
    collections::HashSet,
    env::{self, split_paths, VarError},
    hash::Hash,
    path::PathBuf,
};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix::search_dir;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows::search_dir;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Executable {
    pub name: String,
    pub path: PathBuf,
}

/// Lists all executables on PATH
///
/// **Note:** this does not filter out duplicates
pub fn executables() -> Result<Vec<Executable>, VarError> {
    let path = env::var("PATH")?;
    let paths = split_paths(&path);

    let search_dir = search_dir()?;

    #[cfg(feature = "rayon")]
    let executables = paths
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(search_dir)
        .reduce(Vec::new, |mut a, b| {
            a.extend_from_slice(&b);
            a
        });
    #[cfg(not(feature = "rayon"))]
    let executables = paths.filter_map(search_dir).fold(Vec::new(), |mut a, b| {
        a.extend_from_slice(&b);
        a
    });

    Ok(executables)
}

#[derive(Debug, Clone, Eq)]
struct UniqueExecutable(Executable);

impl PartialEq for UniqueExecutable {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}

impl Hash for UniqueExecutable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}

/// Handles precedence i.e. will only return the first entry in PATH for each executable name
///
/// **Note:** this will never use rayon
///
/// Only partially implemented on Windows. It will choose the correct folder but is not guaranteed
/// to choose the correct executable based on PATHEXT order (e.g. `.exe` before `.bat` when they
/// share a folder).
pub fn unique_executables() -> Result<Vec<Executable>, VarError> {
    let path = env::var("PATH")?;

    Ok(split_paths(&path)
        .filter_map(search_dir()?)
        .flat_map(|dir| dir.into_iter().map(UniqueExecutable))
        .fold(HashSet::new(), |mut set, executable| {
            if !set.contains(&executable) {
                set.insert(executable);
            };
            set
        })
        .into_iter()
        .map(|e| e.0)
        .collect())
}

#[test]
fn test() {
    executables().unwrap();
}
