use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use chrono;
use chrono::prelude::*;
use crate::cli_commands::utils;

pub fn run(title: &str) -> Result<(), std::io::Error> {
    utils::check_krabby_dir()?;

    let mut post_md = File::create(Path::new("./").join("posts").join(&title.to_lowercase()).with_extension("md"))?;

    let buff = format!(
        "<!-- md-meta \n title: {} \n description: Description here \n date: {} \n post-file: {}.html \n--> \n\n",
        title, Utc::now().format("%Y-%m-%d"), title);

    post_md.write_all(buff.as_bytes())?;

    Ok(())
}
