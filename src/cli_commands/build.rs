use crate::cli_commands::utils;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use regex::Regex;
use chrono::NaiveDate;
use comrak::ComrakOptions;
use comrak::markdown_to_html;

struct Replace {
    key: String,
    value: String,
}

struct Post {
    title: String,
    description: String,
    date: String,
    href: String,
}

pub fn replace_vars(content: &str, vars_table: &HashMap<String, String>, verbose: bool) -> String {
    let re = Regex::new(r"\{\s*[A-Za-z0-9_-]+\s*\}").unwrap();
    let mut replaces: Vec<Replace> = Vec::new();

    for captures in re.captures_iter(&content) {
        let var_template = captures.get(0).unwrap().as_str();
        let key = var_template.replace("{", "").replace("}", "").replace(" ", "");

        let var_value = match vars_table.get(&key) {
            Some(a) => a,
            None => { if verbose { println!("Krabby Alert: variable '{}' not found", key)}; continue },
        };

        replaces.push(Replace { key: var_template.to_string(), value: var_value.to_string() });
    }

    let mut result = String::from(content);

    for replace in replaces {
        result = result.replace(&replace.key, &replace.value);
    }

    result
}

fn extract_config(input: json::JsonValue) -> HashMap<String, String> {
    input
        .entries()
        .filter_map(|(key, value)| {
            value.as_str().map(|v| (key.to_string(), v.to_string()))
        })
    .collect()
}

fn extract_meta(input: &str) -> HashMap<String, String> {
    let mut meta = HashMap::new();
    let mut _enabled: bool = false;

    for line in input.lines() {
        if line.contains("<!-- md-meta") {
            _enabled = true;
        } else if line.contains("-->") {
            return meta;
        } else if _enabled == true {
            let strings: Vec<&str> = line.split(":").collect();
            meta.insert(strings[0].trim().to_string(), strings[1].trim().to_string());
        }
    }

    meta
}

fn build_posts(config_vars: HashMap<String, String>) -> Result<Vec<Post>, std::io::Error> {
    let posts_files = fs::read_dir("posts")?;
    let mut posts = vec![];

    for post in posts_files {
        let post = post?;

        let content = fs::read_to_string(&post.path())?;
        let md_meta = extract_meta(&content);

        let mut post_template = fs::read_to_string("post-template.html")?;
        
        post_template = post_template.replace(
            "<md-content>",
            &markdown_to_html(&content, &ComrakOptions::default()),
        );
        
        let mut builded_post = replace_vars(&post_template, &config_vars, false);
        builded_post = replace_vars(&builded_post, &md_meta, true);

        let mut builded_post_path = utils::path_from_string("build/posts");
        builded_post_path.push(post.file_name());
        builded_post_path.set_extension("html");

        fs::write(&builded_post_path, builded_post)?;

        let mut post_href = utils::path_from_string("posts/");
        post_href.push(post.file_name());
        post_href.set_extension("html");

        posts.push(Post {
            title: md_meta["title"].to_string(),
            description: md_meta["description"].to_string(),
            date: md_meta["date"].to_string(),
            href: post_href.into_os_string().into_string().unwrap(),
        });
    }

    Ok(posts)
}

fn build_feed(mut posts: Vec<Post>) -> Result<String, std::io::Error> {
    let mut feed = String::new();

    posts.sort_by_key(|a| NaiveDate::parse_from_str(&a.date as &str, "%d-%m-%Y").unwrap());
    posts.reverse();

    for post in posts {
        feed.push_str(&format!("<div class='post-card'>
  <div class='post-title'>
   <a href={}>{}</a>
  </div>
  <div class='post-date'>{}</div>
  <div class='post-description'>{}</div>
</div>\n\n",
                post.href, post.title, post.date, post.description
                )
            );
    }

    Ok(feed)
}

pub fn run() -> Result<(), std::io::Error> {
    utils::check_krabby_dir()?;
    
    if !Path::new("build").exists() {
        fs::create_dir("build").unwrap();
    }

    if !Path::new("build/posts").exists() {
        fs::create_dir("build/posts").unwrap();
    }

    std::fs::copy("style.css", "build/style.css")?;

    let config_file = fs::read_to_string("krabby-config.json").unwrap();
    let config_json = json::parse(&config_file).unwrap();
    let vars_map = extract_config(config_json);

    let posts = build_posts(vars_map)?;
    let feed = build_feed(posts)?;

    let mut index_template = fs::read_to_string("index-template.html")?;
    index_template = index_template.replace(
        "<posts-feed>",
        &feed,
    );
    let mut builded_path = utils::path_from_string("build");
    builded_path.push("index");
    builded_path.set_extension("html");

    fs::write(&builded_path, index_template)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_vars() {
        let content = "hello {var_1} {var_2}";
        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert(String::from("var_1"), String::from("world"));

        let output = replace_vars(&content, &vars, true);

        assert_eq!(output, "hello world {var_2}");
    }

    #[test]
    fn test_extract_config() {
        let json = json::parse("{\"var_1\": \"world\", \"var_2\": \"hello\"}").unwrap();

        assert_eq!(
            extract_config(json), 
            HashMap::from([
              ("var_1".to_string(), "world".to_string()),
              ("var_2".to_string(), "hello".to_string())
            ])
        )
    }

    #[test]
    fn test_extract_meta() {
        let input = "<!-- md-meta
title: Example post
description: Any description
hello: world
-->
";

        assert_eq!(
            extract_meta(input), 
            HashMap::from([
              ("title".to_string(), "Example post".to_string()),
              ("description".to_string(), "Any description".to_string()),
              ("hello".to_string(), "world".to_string())
            ])
        )

    }

}
