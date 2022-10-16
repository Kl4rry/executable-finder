use std::{env::VarError, os::unix::fs::PermissionsExt, path::PathBuf};

use crate::Executable;

pub fn search_dir() -> Result<fn(PathBuf) -> Option<Vec<Executable>>, VarError> {
    Ok(|path: PathBuf| -> Option<Vec<Executable>> {
        let mut exes = Vec::new();
        if let Ok(dir) = path.read_dir() {
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
    })
}
