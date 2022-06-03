use std::env;
use std::process;

#[path = "cli/new.rs"]
mod new;

#[path = "cli/help.rs"]
mod help;

#[path = "cli/build.rs"]
mod build;

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
        "new" => new::new(),
        "help" => help::help(),
        "build" => build::build(),
        _ => {
            println!("Error: Invalid Command, please use help \n");
            process::exit(1)
        }
    }
}
