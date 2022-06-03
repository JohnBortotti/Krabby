pub fn help() -> Result<(), std::io::Error> {
    print!(
        "
------- Rust-sbg Help ------ 

new                   -> creates a new Rust-ssg project blog
build                 -> build all files to '/build'
help                  -> show this help

for more info         -> https://github.com/JohnBortotti/ssg-rust
----------------------------

"
    );

    Ok(())
}
