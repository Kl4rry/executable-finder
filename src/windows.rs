#[cfg(feature = "unique-executables")]
use std::collections::HashMap;
use std::{collections::HashSet, env, env::VarError, fs};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::Executable;

pub fn executables() -> Result<Vec<Executable>, VarError> {
    let path = env::var("PATH")?;
    let pathext = env::var("PATHEXT")?;

    let exts: HashSet<String> = pathext
        .split(';')
        .map(|s| s.trim_start_matches('.').to_string())
        .collect();

    let search_dir = |path: &&str| -> Option<Vec<Executable>> {
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
    };

    let paths: Vec<&str> = path.split(';').collect();
    #[cfg(feature = "rayon")]
    let executables = paths.par_iter().filter_map(search_dir).reduce(
        || Vec::new(),
        |mut a, b| {
            a.extend_from_slice(&b);
            a
        },
    );
    #[cfg(not(feature = "rayon"))]
    let executables = paths.iter().filter_map(search_dir).fold(
        {
            #[cfg(feature = "unique-executables")]
            {
                HashMap::new()
            }
            #[cfg(not(feature = "unique-executables"))]
            {
                Vec::new()
            }
        },
        |mut a, b| {
            #[cfg(feature = "unique-executables")]
            a.extend(b.into_iter().map(|e| (e.name.clone(), e)));
            #[cfg(not(feature = "unique-executables"))]
            a.extend_from_slice(&b);
            a
        },
    );

    #[cfg(feature = "unique-executables")]
    {
        Ok(executables.into_values().collect())
    }
    #[cfg(not(feature = "unique-executables"))]
    {
        Ok(executables)
    }
}
