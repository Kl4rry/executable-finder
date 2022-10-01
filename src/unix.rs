use std::{env, env::VarError, fs, os::unix::fs::PermissionsExt};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::Executable;

pub fn executables() -> Result<Vec<Executable>, VarError> {
    let path = env::var("PATH")?;

    let search_dir = |path: &&str| -> Option<Executable> {
        if let Ok(dir) = fs::read_dir(path) {
            for entry in dir.flatten() {
                if let Ok(metdata) = entry.metadata() {
                    if !metdata.is_file() {
                        continue;
                    }
                }

                let path = entry.path();
                if let Some(filename) = path.file_name() {
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 != 0 {
                        let exe = Executable {
                            name: filename.to_string_lossy().to_string(),
                            path,
                        };
                        return Some(exe);
                    }
                }
            }
        }
        None
    };

    let paths: Vec<&str> = path.split(':').collect();
    #[cfg(feature = "rayon")]
    let executables = paths.par_iter().filter_map(search_dir).collect();
    #[cfg(not(feature = "rayon"))]
    let executables = paths.iter().filter_map(search_dir).collect();

    Ok(executables)
}
