use std::{collections::HashSet, env, fs};

use crate::ExeError;

pub fn executables() -> Result<Vec<String>, ExeError> {
    let path = env::var("PATH")?;
    let pathext = env::var("PATHEXT")?;

    let exts: HashSet<String> = pathext
        .split(';')
        .into_iter()
        .map(|s| s.trim_start_matches('.').to_string())
        .collect();
    let mut executables = Vec::new();

    for path in path.split(';') {
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => continue,
        };

        for entry in dir {
            let entry = entry?;
            if !entry.metadata()?.is_file() {
                continue;
            }

            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy();
                if exts.contains(&*ext.to_ascii_uppercase()) {
                    if let Some(filename) = path.file_name() {
                        executables.push(filename.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    executables.sort();
    executables.dedup();
    Ok(executables)
}
