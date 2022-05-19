use regex::Regex;
use std::env;
use std::fs;
use std::process;

pub mod md_parser;

// [] executar os commands a partir do diretorio de execução do script
// [] comando help -> listar as paradas pro usuario, como usar, etc.
// [] implementar sistema de cache na função build
// [] implementar code highlight nos posts
// [] melhorar organização e qualidade do code
// [] escrever testes
// [] deixar o cli mais agradavel
// [] comando novo post -> vai gerar um novo arquivo de markdown de acordo com o arquivo de template

fn main() {
    let args = get_command_args();
    execute_command(&args[1], &args).unwrap();
}

fn get_command_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    return match args.get(1) {
        Some(_) => args,
        _ => {
            println!("Error: Invalid Command");
            process::exit(1)
        }
    };
}

fn execute_command(command: &String, args: &Vec<String>) -> Result<(), std::io::Error> {
    match command.to_lowercase().as_str() {
        "new" => match &args.len() {
            3 => new(&args[2]),
            _ => {
                print!("Error: Invalid arguments. Use 'new [path]'");
                process::exit(1)
            }
        },
        // "help" => String::from("help"),
        "build" => build(&args[2]),
        // "post" => String::from("post"),
        _ => {
            println!("Error: Invalid Command");
            process::exit(1)
        }
    }
}

fn new(project_path: &String) -> Result<(), std::io::Error> {
    fs::create_dir_all(project_path)?;
    fs::create_dir(format!("{}/posts/", project_path))?;
    fs::create_dir(format!("{}/build/", project_path))?;
    fs::create_dir(format!("{}/build/posts", project_path))?;

    let template_dir = fs::read_dir("template")?;

    for file in template_dir {
        let file = file.unwrap();
        fs::copy(
            &file.path(),
            format!("{}/{}", project_path, &file.file_name().to_str().unwrap()),
        )?;
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

fn build(project_path: &String) -> Result<(), std::io::Error> {
    let mut index_file_content =
        fs::read_to_string(format!("{}/index-template.html", project_path))?;

    let re: Regex = Regex::new(r"\{\{+\s?[A-Za-z-]+\s?\}\}").unwrap();

    let config_file_content = fs::read_to_string(format!("{}/config.json", project_path))?;
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
        format!("{}/build/index.html", &project_path),
        index_file_content,
    )?;

    let posts_files = fs::read_dir(format!("{}/posts", project_path))?;
    for post in posts_files {
        let post = post?;
        let md_content = fs::read_to_string(&post.path())?;

        // [] tratar essa gambiarra aqui (sem esses replace pf né)
        let post_build_path = format!("{}/build/posts/{:?}", project_path, post.file_name())
            .replace('"', "")
            .replace(".md", ".html");

        let html_template = fs::read_to_string(format!("{}/post-template.html", project_path))?;

        let builded_post = html_template.replace(
            "<md-content>",
            &md_parser::parse_string(&md_content).to_string(),
        );

        fs::write(post_build_path, builded_post)?;
    }

    Ok(())
}
