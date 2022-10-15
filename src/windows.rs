use std::{collections::HashSet, env, env::VarError, fs};

use crate::Executable;

pub fn split_paths(path: &str) -> impl Iterator<Item = &str> {
    path.split(';')
}

pub fn search_dir() -> Result<impl Fn(&str) -> Option<Vec<Executable>>, VarError> {
    let pathext = env::var("PATHEXT")?;

    let exts: HashSet<String> = pathext
        .split(';')
        .map(|s| s.trim_start_matches('.').to_string())
        .collect();

    Ok(move |path: &str| -> Option<Vec<Executable>> {
        let mut exes = Vec::new();
        if let Ok(dir) = fs::read_dir(path) {
            for entry in dir.flatten() {
                if let Ok(metdata) = entry.metadata() {
                    if !metdata.is_file() {
                        continue;
                    }
                }

                let path = entry.path();
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy();
                    if exts.contains(&*ext.to_ascii_uppercase()) {
                        if let Some(filename) = path.file_name() {
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
