use crate::cli_commands::utils;

pub fn run() -> Result<(), std::io::Error> {
    utils::check_krabby_dir()?;

    Ok(())
}
