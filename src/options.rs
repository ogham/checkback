use std::env::args;
use std::path::PathBuf;

pub fn input_paths() -> Vec<PathBuf> {
    args().skip(1)
        .map(PathBuf::from)
        .collect()
}

