use regex::Regex;
use std::env;
use std::fs;
use std::process;

pub mod md_parser;
pub mod utils;

// --------------
// TODO
// [] ajustar o css dos templates
// [] ao fazer o build, pegar todos os posts e plotar no index com a paginação (criar cabeçalho com infos no markdown)
// -------------
 
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
    // ------------------
    // TODO
    // [] adiciona um template de markdown com os metadados já inseridos como exemplo
    // ------------------
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
    // lendo template_html
    let mut index_file_content =
        fs::read_to_string(utils::path_from_string(&"blog/index-template.html"))?;

    // buildando o index.html
    // substituindo variaveis -> config.json
    // -------------------------
    // TODO
    // [] Isolar isso em uma função retornando o conteudo para seguir o fluxo
    // [] Implementar um hash para fazer cache desse build (mais simples que os posts, dá pra
    // começar por aqui)
    // -------------------------
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

    // buildando os posts
    // -------------------
    // TODO
    // [] Organizar melhor essas funções, e implementar o sistema de cache
    // ------------------
    // lendo posts
    let posts_files = fs::read_dir(utils::path_from_string(&"blog/posts/"))?;

    for post in posts_files {
        let post = post?;
        // texto do arquivo
        let md_content = fs::read_to_string(&post.path())?;

        // arquivo template de post
        let mut html_post_template =
            fs::read_to_string(utils::path_from_string(&"blog/post-template.html"))?;

        // metadados
        let _meta = md_parser::extract_meta(&md_content);

        // sobrescrevendo os metadados
        // -------------
        // TODO
        // [] Melhorar isso aqui e remover o cabeçalho de metadados do arquivo final
        // [] Adicionar os metadados no texto vindo do markdown (provavelmente passando esse
        // função para baixo no fluxo)
        // -------------
        let mut post_replace: Vec<Replace> = Vec::new();

        for res in re.captures_iter(&html_post_template) {
            let variable_key = res.get(0).unwrap().as_str();

            let key = variable_key
                .replace("{", "")
                .replace("}", "")
                .replace(" ", "");

            let config_value = _meta.get(&key.as_str()).unwrap();

            post_replace.push(Replace::new(
                variable_key.to_string(),
                config_value.to_string(),
            ));
        }

        for replace in post_replace {
            html_post_template = html_post_template.replace(&replace.key, &replace.value);
        }

        // buildando markdown e colocando no html
        let builded_post = html_post_template.replace(
            "<md-content>",
            &md_parser::parse_string(&md_content).to_string(),
        );

        // salvando resultado no html
        let mut builded_post_path = utils::path_from_string("blog/build/posts");
        builded_post_path.push(post.file_name());
        builded_post_path.set_extension("html");

        fs::write(&builded_post_path, builded_post)?;
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
