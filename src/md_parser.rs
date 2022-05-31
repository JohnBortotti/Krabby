use std::collections::HashMap;
use std::fs::File;
use std::str;

pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
    open_tags: Vec<String>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
            open_tags: vec![],
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len() - 2
    }

    pub fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(characters) => {
                self.cursor += 1;

                Some(characters)
            }
            None => None,
        }
    }

    pub fn overwrite(&mut self, replace_len: usize, string: &str) {
        let start_offset = if self.cursor < string.len() {
            0
        } else {
            self.cursor - replace_len
        };
        self.characters
            .splice(start_offset..self.cursor, string.chars());
    }

    pub fn copy(&mut self) {
        self.characters
            .push(self.characters.get(self.cursor).copied().unwrap())
    }

    pub fn get_result_string(&mut self) -> String {
        let result = &self.characters;
        result.into_iter().collect()
    }

    pub fn overwrite_tags(
        &mut self,
        chars: &str,
        open_tag: &str,
        close_tag: &str,
        inline_tag: bool,
    ) {
        self.cursor = 0;
        let mut success = 0;
        let mut is_inline_open: bool = false;

        let token_length = chars.chars().count();
        loop {
            if self.cursor > 0 && self.is_done() {
                break;
            }

            for char in chars.chars() {
                match self.characters.get(self.cursor) {
                    Some(_) => {
                        if &char == self.characters.get(self.cursor).unwrap() {
                            success += 1;
                            self.pop();
                            if success == token_length {
                                if inline_tag == false {
                                    self.overwrite(chars.len(), open_tag);
                                    self.open_tags.push(String::from(close_tag));
                                    success = 0;
                                } else if inline_tag == true && is_inline_open == false {
                                    is_inline_open = true;
                                    self.overwrite(chars.len(), open_tag);
                                } else if inline_tag == true && is_inline_open == true {
                                    is_inline_open = false;
                                    self.overwrite(chars.len(), close_tag);
                                }
                            }
                        } else {
                            if self.is_done() {
                                return;
                            }

                            if self.characters.get(self.cursor).unwrap() == &'\n'
                                && self.open_tags.len() > 0
                            {
                                let html_tag = self.open_tags.remove(0);
                                self.overwrite(0, &html_tag)
                            }
                            success = 0;
                            self.pop();
                        }
                    }
                    _ => {
                        success = 0;
                        self.pop();
                    }
                }
            }
        }
    }

    pub fn scan_chars(&mut self) {
        self.overwrite_tags("####", "<h4>", "</h4>", false);
        self.overwrite_tags("###", "<h3>", "</h3>", false);
        self.overwrite_tags("##", "<h2>", "</h2>", false);
        self.overwrite_tags("#", "<h1>", "</h1>", false);
        self.overwrite_tags("```", "<pre><code>", "</code></pre>", true);
        self.overwrite_tags("***", "<strong>", "</strong>", true);
        self.overwrite_tags("*", "<em>", "</em>", true);
        self.overwrite_tags("\n\n", "<br>", "", false);
        self.overwrite_tags("\n", "\n", "\n", false);
    }
}

pub fn parse_string(string: &str) -> String {
    let mut scanner = Scanner::new(string);

    scanner.scan_chars();
    scanner.get_result_string()
}

pub fn extract_meta(input: &str) -> HashMap<&str, &str> {
    let mut meta = HashMap::new();
    let mut _enabled: bool = false;

    for line in input.lines() {
        if line == "---" {
            if _enabled == false {
                _enabled = true;
                continue;
            } else {
                break;
            }
        } else {
            let strings: Vec<&str> = line.split(":").collect();
            meta.insert(strings[0].trim(), strings[1].trim());
        }
    }

    meta
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::md_parser;

    #[test]
    fn should_return_cursor() {
        let mut scanner = Scanner::new("any");
        scanner.cursor += 3;
        assert_eq!(scanner.cursor(), 3);
    }

    #[test]
    fn should_peek_char() {
        let scanner = Scanner::new("hello world");
        assert_eq!(scanner.peek().unwrap(), &'h');
    }

    #[test]
    fn should_pop_next_char() {
        let mut scanner = Scanner::new("rust");
        assert_eq!(scanner.pop().unwrap(), &'r');
    }

    #[test]
    fn should_overwrite_correctly() {
        let mut scanner = Scanner::new("rust");
        scanner.overwrite(0, "javascript");
        assert_eq!(scanner.pop().unwrap(), &'j');
    }

    #[test]
    fn should_parse_md_correcly() {
        let parsed = md_parser::parse_string(
            "# Title
  ",
        );
        assert_eq!(parsed, "<h1> Title</h1>\n  ");
    }
}
