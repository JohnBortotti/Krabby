use std::fs;

#[path = "../utils.rs"]
mod utils;

/*
 * Create `blog` dir with template files and default directories
*/
pub fn new() -> Result<(), std::io::Error> {
    let mut project_path = utils::path_from_string("blog");

    fs::create_dir_all(&project_path)?;

    project_path.push("posts");
    fs::create_dir(&project_path)?;

    project_path.push("example-md.md");
    fs::write(
        &project_path,
        b"
<!-- md-meta
title: post title
description: any description
date: 1-1-2077
-->

# Write you markdown here
",
    )?;

    project_path.pop();

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
