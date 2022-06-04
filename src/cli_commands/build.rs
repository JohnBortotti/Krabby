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

struct BuildedPost {
    title: String,
    description: String,
    date: String,
    href: String,
}

impl BuildedPost {
    fn new(title: String, description: String, date: String, href: String) -> BuildedPost {
        Self {
            title,
            description,
            date,
            href,
        }
    }
}

/*
 * Replace all text variables with values declared on config.json
 */
fn replace_config_variables(content: &str) -> String {
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
fn replace_meta_variables(content: &str, _meta: &HashMap<&str, &str>) -> String {
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
 * insert into index.html all builded posts (posts feed)
 */
fn build_posts_feed(posts: Vec<BuildedPost>) -> Result<(), std::io::Error> {
    let mut posts_feed_buffer = String::new();

    for post in posts {
        posts_feed_buffer.push_str(&format!(
            "<div class='post-card'>
  <div class='post-title'>
  <a href={}>{}</a>
  </div>
  <div class='post-date'>{}</div>
  <div class='post-description'>{}</div>
</div>",
            post.href, post.title, post.date, post.description
        ));
        posts_feed_buffer.push_str("\n\n")
    }

    let index_file = fs::read_to_string(utils::path_from_string(&"blog/build/index.html"))?;

    let index_with_feed = index_file.replace("<posts-feed>", &posts_feed_buffer);

    fs::write(
        utils::path_from_string(&"blog/build/index.html"),
        replace_config_variables(&index_with_feed),
    )?;

    Ok(())
}

/*
 * Build index-template and all md posts, replacing variables
 * and translating the markdown content to html
 */
pub fn run_command() -> Result<(), std::io::Error> {
    let mut index_file_content =
        fs::read_to_string(utils::path_from_string(&"blog/index-template.html"))?;

    let configs =
        json::parse(&fs::read_to_string(utils::path_from_string("blog/config.json")).unwrap())
            .unwrap();

    let mut theme_path = utils::path_from_string("themes/");
    let _ = &theme_path.push(configs["theme"].as_str().unwrap());
    let theme_css = fs::read_to_string(theme_path).unwrap();

    index_file_content.push_str("\n\n<style>");
    index_file_content.push_str(&theme_css);
    index_file_content.push_str("\n\n</style>");

    fs::write(
        utils::path_from_string(&"blog/build/index.html"),
        replace_config_variables(&index_file_content),
    )?;

    let posts_files = fs::read_dir(utils::path_from_string(&"blog/posts/"))?;

    let mut posts_feed: Vec<BuildedPost> = Vec::new();

    for post in posts_files {
        let post = post?;

        let md_content = fs::read_to_string(&post.path())?;
        let md_meta = md_parser::extract_meta(&md_content);

        let html_post_template =
            fs::read_to_string(utils::path_from_string(&"blog/post-template.html"))?;

        let html_with_post_content =
            html_post_template.replace("<md-content>", &md_parser::parse_string(&md_content));

        let mut builded_post = replace_config_variables(&html_with_post_content);
        builded_post = replace_meta_variables(&builded_post, &md_meta);

        let configs =
            json::parse(&fs::read_to_string(utils::path_from_string("blog/config.json")).unwrap())
                .unwrap();

        let mut theme_path = utils::path_from_string("themes/");
        let _ = &theme_path.push(configs["theme"].as_str().unwrap());
        let theme_css = fs::read_to_string(theme_path).unwrap();

        builded_post.push_str("\n\n<style>");
        builded_post.push_str(&theme_css);
        builded_post.push_str("\n\n</style>");

        let mut builded_post_path = utils::path_from_string("blog/build/posts");
        builded_post_path.push(post.file_name());
        builded_post_path.set_extension("html");

        fs::write(&builded_post_path, builded_post)?;

        let mut post_href = utils::path_from_string("posts/");
        post_href.push(post.file_name());
        post_href.set_extension("html");

        posts_feed.push(BuildedPost::new(
            md_meta["title"].to_string(),
            md_meta["description"].to_string(),
            md_meta["date"].to_string(),
            post_href.into_os_string().into_string().unwrap(),
        ));
    }

    build_posts_feed(posts_feed)?;

    Ok(())
}
