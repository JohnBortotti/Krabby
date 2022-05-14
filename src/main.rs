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
//    [x] Criar arquivo de configuração
//    [x] Criar diretorio posts
//    [x] Criar arquivo template post.html

// [x] definir o formato de arquivo para o post => MARKDOWN meu parsero
// [ ] construir um esboço do parser de markdown

// [] comando help -> listar as paradas
// [] comando build -> vai buildar as paradas, substituir variaveis, formatar blocos de código, e TALVEZ markdown, e TALVEZ algum cache
// [] comando novo post -> vai gerar um novo arquivo de post de acordo com o arquivo de template

fn main() {
    // let args = get_command_args();
    md_parser::parse_string(" # Title Só testando ## mano");
    // execute_command(&args[1], &args).unwrap();
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
        // "build" => String::from("build"),
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
