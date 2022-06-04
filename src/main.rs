use std::env;
use std::process;

pub mod cli_commands;

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
        "new" => cli_commands::new::run_command(),
        "help" => cli_commands::help::run_command(),
        "build" => cli_commands::build::run_command(),
        _ => {
            println!("Error: Invalid Command, please use help \n");
            process::exit(1)
        }
    }
}
