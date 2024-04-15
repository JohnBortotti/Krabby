use std::env;
use std::process;

pub mod cli_commands;

fn main() {
    let args = get_command_args();

    if args.len() < 1 {
        println!("Error: Invalid Command, please use help \n");
        process::exit(1)
    }

    match args[1].to_lowercase().as_str() {
        "init"  => { 
            if args.len() < 3 {
                println!("Error: you need to specify a project name");
                process::exit(1)
            }
            cli_commands::init::run(&args[2])
        },
        "post"  => { 
            if args.len() < 3 {
                println!("Error: you need to specify a post name");
                process::exit(1)
            }
            cli_commands::post::run(&args[2])
        },
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
