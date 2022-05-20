use std::path::PathBuf;

pub fn path_from_string(path: &str) -> PathBuf {
    let mut pathbuf = PathBuf::new();

    pathbuf.push(path);
    pathbuf
}
