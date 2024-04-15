use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::path::Path;
use std::env;
use std::process;

pub fn path_from_string(path: &str) -> PathBuf {
    let mut pathbuf = PathBuf::new();

    pathbuf.push(path);
    pathbuf
}

pub fn copy_full_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    std::fs::create_dir_all(&dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;

        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_full_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

pub fn check_krabby_dir() -> Result<(), std::io::Error>{
     let mut path = env::current_dir()?;
     path.push("krabby-config.json");

     match File::open(path) {
         Ok(_) => Ok(()),
         Err(_) => {
             println!("Error: Invalid Krabby project, check you current directory");
             process::exit(1);
         }
     }
}
