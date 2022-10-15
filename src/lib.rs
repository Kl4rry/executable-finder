#[cfg(feature = "rayon")]
use rayon::prelude::*;

use std::{
    collections::HashMap,
    env::{self, VarError},
    path::PathBuf,
};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix::{search_dir, split_path};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows::{search_dir, split_path};

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
    let paths = split_path(&path);

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

/// Handles precedence i.e. an entry later in PATH will override an earlier one
///
/// **Note:** this will never use rayon
pub fn unique_executables() -> Result<HashMap<String, Executable>, VarError> {
    let path = env::var("PATH")?;

    Ok(split_path(&path)
        .filter_map(search_dir()?)
        .fold(HashMap::new(), |mut a, b| {
            a.extend(b.into_iter().map(|e| (e.name.clone(), e)));
            a
        }))
}

#[test]
fn test() {
    executables().unwrap();
}
