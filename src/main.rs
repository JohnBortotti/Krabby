use std::env;
use std::process;

pub mod cli_commands;

fn main() {
    let args = get_command_args();

    match args[1].to_lowercase().as_str() {
        "init"  => cli_commands::init::run(&args[2]),
        "post"  => cli_commands::post::run(&args[2]),
        "build" => cli_commands::build::run(),
        "help"  => cli_commands::help::run(),
        _ => {
            println!("Error: Invalid Command, please use help \n");
            process::exit(1)
        }
    }.unwrap()
}

fn get_command_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();


    return match args.get(1) {
        Some(_) => args,
        None => vec![]
    };
}
