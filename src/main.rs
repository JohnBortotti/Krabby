use regex::Regex;
use std::env;
use std::fs;
use std::process;

pub mod md_parser;

// Aqui será a entrada do role, sera o cli que vai receber e executar os commands

// [x] Pegar o argumentos do processo
// [x] Validar argumento

// [] Comando para criar o blog -> vai criar uma pasta com os arquivos iniciais
//    [x] Criar diretório
//    [x] Criar arquivo template index.html
//    [] Criar arquivo de configuração -> definir o formato
//    [x] Criar diretorio posts
//    [x] Criar arquivo template post.html
//    [] Criar diretorio a partir do diretorio de execução do script

// [x] definir o formato de arquivo para o post => MARKDOWN meu parsero
// [x] construir um esboço do parser de markdown

// [] comando help -> listar as paradas pro usuario, como usar, etc.
// [] comando build -> vai buildar as paradas, substituir variaveis, formatar blocos de código, e TALVEZ markdown, e TALVEZ algum cache
//   [x] substiruir variaveis no arquivo build/index
//   [] pegar o markdown processado, e concatenar dentro do html
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
    )
    .unwrap();

    Ok(())
}
