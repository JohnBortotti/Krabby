use std::str;

pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
    open_tags: Vec<String>,
    parsed: String,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
            open_tags: vec![],
            parsed: String::new(),
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

    pub fn write(&mut self, string: &str) {
        self.parsed.push_str(string)
    }

    pub fn copy(&mut self) {
        let char = self.characters.get(self.cursor);
        self.parsed.push(char.copied().unwrap())
    }

    pub fn overwrite_tags(&mut self, chars: &str, open_tag: &str, close_tag: &str) {
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
                            if success == token_length {
                                self.write(open_tag);
                                self.open_tags.push(String::from(close_tag));
                            }
                        } else {
                            if self.is_done() {
                                return;
                            }
                            self.copy();
                            self.pop();
                        }
                    }
                    _ => {
                        self.copy();
                    }
                }
            }
        }
    }

    pub fn scan_chars(&mut self) {
        self.overwrite_tags(" # ", "<h1>", "</h1>");
        self.cursor = 0;
        self.overwrite_tags(" ## ", "<h2>", "</h2>");

        println!("a sentenca final foi: {}", self.parsed);
        println!("as tags abertas foram: {:?}", self.open_tags);
    }
}

pub fn parse_string(string: &str) -> String {
    let mut scanner = Scanner::new(string);

    // loop {
    scanner.scan_chars();

    // if scanner.cursor() > 0 && scanner.is_done() {
    // println!("{:?}", scanner.open_tags);
    // break;
    // }
    // }

    scanner.parsed
}
