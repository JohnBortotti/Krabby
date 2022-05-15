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

    pub fn overwrite_tags(&mut self, chars: &str, open_tag: &str, close_tag: &str) {
        self.cursor = 0;
        let mut success = 0;
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
                            if success == token_length
                                && self.characters.get(self.cursor).unwrap() == &' '
                            {
                                self.overwrite(chars.len(), open_tag);
                                self.open_tags.push(String::from(close_tag));
                                success = 0;
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
        self.overwrite_tags("#", "<h1>", "</h1>");
        self.overwrite_tags("##", "<h2>", "</h2>");
        self.overwrite_tags("###", "<h3>", "</h3>");
        self.overwrite_tags("\n", "<br>", "br>");

        println!("a sentenca final foi: {:?}", self.get_result_string());
    }
}

pub fn parse_string(string: &str) -> String {
    let mut scanner = Scanner::new(string);

    scanner.scan_chars();
    scanner.get_result_string()
}
