pub fn run() -> Result<(), std::io::Error> {
    print!(
        "
------- Krabby Help ------ 

init [project_name]   -> Create a new krabby project 
post [post_name]      -> creates a new Post
help                  -> show this help
build                 -> build project

more info             -> https://github.com/JohnBortotti/Krabby
----------------------------

"
    );

    Ok(())
}
