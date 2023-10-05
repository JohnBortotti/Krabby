# Krabby
Krabby is a minimalistic static blog generator written in Rust. (WIP)

## How to install
Just clone this repo
```
git clone https://github.com/JohnBortotti/Krabby.git
```

## How to use

#### Commands
```
init {project_name} -> creates a new Krabby project
post {post_name}    -> creates a new Post
build               -> build project
help                -> show this help
```

#### Project 
After creating a new Krabby project, the following structure will be created:
```
index-template.html -> blog index page template
krabby-config.json  -> configs
posts/              -> posts dir
post-template.html  -> blog post page template
style.css           -> blog css

```


#### Variables
You can create and set some variables wich will be replaced in the html files at building stage,
you can define global variables (used throught all the blog), or post scoped variables, wich are used only
inside the post.

Defining global variables, inside krabby-config.json:
```
"author": "John Bortoti" -> setting a new value
"hello": "world" -> creating a new variable
```

Defining post variables, inside post markdown:
```
title: Example post           
description: Any description  
date: 1-1-2077               
hello: world
```

#### Using variables
Using variables is pretty straightforward, using the notation:
```{variable_key}```

example:
```
hello, my name is {author}, and the blog name is {blog-name}
```

#### Contributing
Feel free to contribute with the project, writing PRs, Issues, etc.
