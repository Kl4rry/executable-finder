use std::{env, fs, os::unix::fs::PermissionsExt};

use crate::{ExeError, Executable};

pub fn executables() -> Result<Vec<Executable>, ExeError> {
    let path = env::var("PATH")?;
    let mut executables: Vec<Executable> = Vec::new();

    for path in path.split(':') {
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => continue,
        };

        for entry in dir {
            let entry = entry?;
            let metadata = entry.metadata()?;
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
                    executables.push(exe);
                }
            }
        }
    }

    executables.sort();
    executables.dedup();
    Ok(executables)
}
