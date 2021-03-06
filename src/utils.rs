use std::path::PathBuf;

pub fn path_from_string(path: &str) -> PathBuf {
    let mut pathbuf = PathBuf::new();

    pathbuf.push(path);
    pathbuf
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_pathbuf_from_string() {
        let path = path_from_string("any/path");

        let mut expected = PathBuf::new();
        expected.push("any/path");

        assert_eq!(path, expected);
    }
}
