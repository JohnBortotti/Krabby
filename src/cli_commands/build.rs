use super::md_parser;
use super::utils;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

struct Replace {
    key: String,
    value: String,
}

impl Replace {
    fn new(key: String, value: String) -> Replace {
        Self { key, value }
    }
}

/*
 * Replace all text variables with values declared on config.json
 */
pub fn replace_config_variables(content: &str) -> String {
    let re: Regex = Regex::new(r"\{\{+\s?[A-Za-z-]+\s?\}\}").unwrap();

    let config_file_content =
        fs::read_to_string(utils::path_from_string(&"blog/config.json")).unwrap();
    let configs = json::parse(&config_file_content).unwrap();
    let mut replaces_vec: Vec<Replace> = Vec::new();
    for res in re.captures_iter(&content) {
        let variable_notation_key = res.get(0).unwrap().as_str();

        let key = variable_notation_key
            .replace("{", "")
            .replace("}", "")
            .replace(" ", "");

        let config_value = &configs[key];

        if !config_value.is_null() {
            replaces_vec.push(Replace::new(
                variable_notation_key.to_string(),
                config_value.to_string(),
            ))
        }
    }

    let mut result = String::from(content);

    for replace in replaces_vec {
        result = result.replace(&replace.key, &replace.value);
    }

    result
}

/*
 * Replace all text variables with values declared on post _meta header
 */
pub fn replace_meta_variables(content: &str, _meta: HashMap<&str, &str>) -> String {
    let re: Regex = Regex::new(r"\{\{+\s?[A-Za-z-]+\s?\}\}").unwrap();
    let mut replaces_vec: Vec<Replace> = Vec::new();

    for res in re.captures_iter(&content) {
        let meta_key = res.get(0).unwrap().as_str();

        let key = meta_key.replace("{", "").replace("}", "").replace(" ", "");

        replaces_vec.push(Replace::new(
            meta_key.to_string(),
            _meta[&key as &str].to_string(),
        ))
    }

    let mut result = String::from(content);
    for replace in replaces_vec {
        result = result.replace(&replace.key, &replace.value);
    }

    result
}

/*
 * Build index-template and all md posts, replacing variables
 * and translating the markdown content to html
 */
pub fn run_command() -> Result<(), std::io::Error> {
    let index_file_content =
        fs::read_to_string(utils::path_from_string(&"blog/index-template.html"))?;

    fs::write(
        utils::path_from_string(&"blog/build/index.html"),
        replace_config_variables(&index_file_content),
    )?;

    let posts_files = fs::read_dir(utils::path_from_string(&"blog/posts/"))?;

    for post in posts_files {
        let post = post?;

        let md_content = fs::read_to_string(&post.path())?;
        let _md_meta = md_parser::extract_meta(&md_content);

        let _html_post_template =
            fs::read_to_string(utils::path_from_string(&"blog/post-template.html"))?;

        let _html_with_post_content =
            _html_post_template.replace("<md-content>", &md_parser::parse_string(&md_content));

        let mut _builded_post = replace_config_variables(&_html_with_post_content);
        let mut _builded_post = replace_meta_variables(&_builded_post, _md_meta);

        let _configs =
            json::parse(&fs::read_to_string(utils::path_from_string("blog/config.json")).unwrap())
                .unwrap();

        let mut theme_path = utils::path_from_string("themes/");
        let _ = &theme_path.push(_configs["theme"].as_str().unwrap());
        let theme_css = fs::read_to_string(theme_path).unwrap();

        _builded_post.push_str("\n\n<style>");
        _builded_post.push_str(&theme_css);
        _builded_post.push_str("\n\n</style>");

        let mut builded_post_path = utils::path_from_string("blog/build/posts");
        builded_post_path.push(post.file_name());
        builded_post_path.set_extension("html");

        fs::write(&builded_post_path, _builded_post)?;
    }

    Ok(())
}
