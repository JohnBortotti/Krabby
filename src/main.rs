use regex::Regex;
use std::env;
use std::fs;
use std::process;

pub mod md_parser;
pub mod utils;

// [] implementar sistema de cache na função build
// [] criar lista -> melhorar organização e qualidade do code (principalmente do parser maluco)
// [] ajustar os templates para subir o blog
// [] definir uma forma de cabecalho com infos para os posts (dentro do arquivo md)
// [] colocar no index-template.html a tag para dar replace com os posts
// [] adicionar tags de SEO nos teplates html
// [] adicionar sistema de paginacao dos posts
 
fn main() {
    let args = get_command_args();
    execute_command(&args[1]).unwrap();
}

fn get_command_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    return match args.get(1) {
        Some(_) => args,
        _ => {
            println!("Error: Invalid Command, please use help \n");
            process::exit(1)
        }
    };
}

fn execute_command(command: &String) -> Result<(), std::io::Error> {
    match command.to_lowercase().as_str() {
        "new" => new(),
        "help" => help(),
        "build" => build(),
        _ => {
            println!("Error: Invalid Command, please use help \n");
            process::exit(1)
        }
    }
}

fn new() -> Result<(), std::io::Error> {
    let mut project_path = utils::path_from_string("blog");

    fs::create_dir_all(&project_path)?;

    project_path.push("posts");
    fs::create_dir(&project_path)?;

    project_path.pop();
    project_path.push("build");
    fs::create_dir(&project_path)?;

    project_path.push("posts");
    fs::create_dir(&project_path)?;

    project_path.pop();
    project_path.pop();

    let template_dir = fs::read_dir("./template")?;

    for file in template_dir {
        let file = file.unwrap();

        project_path.push(file.file_name().to_str().unwrap());

        fs::copy(&file.path(), &project_path)?;

        project_path.pop();
    }

    Ok(())
}

struct Replace {
    key: String,
    value: String,
}

impl Replace {
    pub fn new(key: String, value: String) -> Replace {
        Self { key, value }
    }
}

fn build() -> Result<(), std::io::Error> {
    let mut index_file_content =
        fs::read_to_string(utils::path_from_string(&"blog/index-template.html"))?;

    let re: Regex = Regex::new(r"\{\{+\s?[A-Za-z-]+\s?\}\}").unwrap();

    let config_file_content = fs::read_to_string(utils::path_from_string(&"blog/config.json"))?;

    let configs = json::parse(&config_file_content).unwrap();

    let mut replaces: Vec<Replace> = Vec::new();

    for res in re.captures_iter(&index_file_content) {
        let variable_key = res.get(0).unwrap().as_str();

        let key = variable_key
            .replace("{", "")
            .replace("}", "")
            .replace(" ", "");

        let config_value = &configs[&key];

        replaces.push(Replace::new(
            variable_key.to_string(),
            config_value.to_string(),
        ));
    }

    for replace in replaces {
        index_file_content = index_file_content.replace(&replace.key, &replace.value);
    }

    fs::write(
        utils::path_from_string(&"blog/build/index.html"),
        index_file_content,
    )?;

    let posts_files = fs::read_dir(utils::path_from_string(&"blog/posts/"))?;

    for post in posts_files {
        let post = post?;
        let md_content = fs::read_to_string(&post.path())?;

        let html_post_template =
            fs::read_to_string(utils::path_from_string(&"blog/post-template.html"))?;

        let builded_post = html_post_template.replace(
            "<md-content>",
            &md_parser::parse_string(&md_content).to_string(),
        );

        let mut builded_post_path = utils::path_from_string("blog/build/posts");
        builded_post_path.push(post.file_name());
        builded_post_path.set_extension("html");

        fs::write(builded_post_path, builded_post)?;
    }

    Ok(())
}

fn help() -> Result<(), std::io::Error> {
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
