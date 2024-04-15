use std::env;
use crate::cli_commands::utils::copy_full_dir;

pub fn run(project_name: &str) -> Result<(), std::io::Error> {
    let mut target_path = env::current_dir()?;
    target_path.push(project_name);

    copy_full_dir("template", target_path)?;
    println!("Krabby project created: {}", project_name);

    Ok(())
}
