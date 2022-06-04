pub fn run_command() -> Result<(), std::io::Error> {
    print!(
        "
------- Rust-sbg Help ------ 

build                 -> build all files to '/build'
help                  -> show this help

for more info         -> https://github.com/JohnBortotti/ssg-rust
----------------------------

"
    );

    Ok(())
}
