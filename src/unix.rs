use std::{env, env::VarError, fs, os::unix::fs::PermissionsExt};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::Executable;

pub fn executables() -> Result<Vec<Executable>, VarError> {
    let path = env::var("PATH")?;

    let search_dir = |path: &&str| -> Option<Vec<Executable>> {
        let mut exes = Vec::new();
        if let Ok(dir) = fs::read_dir(path) {
            for entry in dir.flatten() {
                // We need to call metadata on the path to follow symbolic links
                if let Ok(metadata) = entry.path().metadata() {
                    if !metadata.is_file() {
                        continue;
                    }

                    let path = entry.path();
                    if let Some(filename) = path.file_name() {
                        let permissions = metadata.permissions();
                        if permissions.mode() & 0o111 != 0 {
                            let exe = Executable {
                                name: filename.to_string_lossy().to_string(),
                                path,
                            };

                            exes.push(exe);
                        }
                    }
                }
            }
        }

        if exes.is_empty() {
            None
        } else {
            Some(exes)
        }
    };

    let paths: Vec<&str> = path.split(':').collect();
    #[cfg(feature = "rayon")]
    let executables = paths.par_iter().filter_map(search_dir).reduce(
        || Vec::new(),
        |mut a, b| {
            a.extend_from_slice(&b);
            a
        },
    );
    #[cfg(not(feature = "rayon"))]
    let executables = paths
        .iter()
        .filter_map(search_dir)
        .fold(Vec::new(), |mut a, b| {
            a.extend_from_slice(&b);
            a
        });

    Ok(executables)
}

#[test]
fn test() {
    executables();
}
