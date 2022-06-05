# Krabby
Krabby is a minimalistic static blog generator written in Rust.

## How to install
Just clone this repo
```
git clone https://github.com/JohnBortotti/Krabby.git
```

## How to use
#### Files
After clonning the repo, we have:
```
blog/        -> Here is your blog with file configs, html templates and markdown posts
src/         -> Krabby source code (feel free to explore and contribute to the project)
themes/      -> CSS themes, you can use the default themes or create you owns here
```
Inside the `/blog`:
```
build/               -> Builded files, just drop this files inside your web server
posts/               -> All your markdown posts
config.json          -> Blog configs
index-template.html  -> The template for index.html page
post-template.html   -> The template for posts page
```

#### Configs - `config.json`
Here we can configure some variables and infos about the blog, you can add or remove any key and set any value you want. The only required key is the `theme` wich will be used to resolve your blog css theme:

```
config.json

"theme": "dark.css" \\ this is the default theme, you can modify it or create others
```

lets add another variable, now i want a one to handle my blog name:

```
config.json

"theme": "dark.css"
"blog-name": "Rust Krabby" \\ i can set any key and any value here, and use later on templates
```

After configuring the `config.json`, we can use this variables on templates:

#### Templates
To use variables iniside our templates and posts, we can use the variable notation: `{{ variable-key }}`:

```
index-template.html

<div class="navbar">{{ blog-name }}</div>
```

After the build step, the variable will be replaced with the value daclared on config.json. And clearly you can use the same for `post-template.html` and any post markdown on `blog/posts`.

#### Posts
To create a new post, just create a new markdown file at `blog/posts` like this:

```
example-post.md

<!-- md-meta
title: Example post           (required key)
description: Any description  (required key)
date: 1-1-2077                (required key)
hello: world
(here you can add any other keys, like config.json but specific for each post, like tags, author, ...)
-->

Write your post here

{{ blog-name }} \\ use variables the same way, you can use variables both from config.json than from meta header 
{{ hello }} 

```

#### Commands
Finally, after setup your configs, and posts, you will need to build the blog:

```
cargo run build
```
If you need some help, you can use:

```
cargo run help

```
#### Contributing
Feel free to contribute with the project, writing PRs, Issues, etc.
