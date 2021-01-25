use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    Eof,
    VarPrefix,
    LeftBracket,
    RightBracket,
    Equal,
    String,
    Name,
    Print,
    Ignored,
    None,
}

#[derive(Clone, Debug)]
pub struct Token(pub TokenType, pub usize, pub Option<String>);

pub struct Lexer {
    source: Vec<String>,
    line: usize,
    ptr: usize,
    pub next_token_info: TokenType,
    next_token: Token,
    key_words: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut hash = HashMap::new();
        hash.insert("print".to_string(), TokenType::Print);
        Lexer {
            source: source.split('\n').map(|s| s.to_string()).collect(),
            line: 0,
            ptr: 0,
            next_token_info: TokenType::None,
            next_token: Token(TokenType::None, 0, None),
            key_words: hash,
        }
    }

    fn scan_name(&self, s: &str, start: usize) -> (usize, String) {
        let codes = &s[start..];
        let mut i = 0;
        for c in codes.chars() {
            match c {
                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => i += 1,
                _ => break,
            }
        }
        (i, codes[..i].to_string())
    }

    fn scan_string(&self, s: &str, start: usize) -> (usize, String) {
        let codes = &s[start..];
        let mut i = 0;
        for c in codes.chars() {
            match c {
                '"' => {
                    i += 1;
                    return (i, codes[..i - 1].to_string());
                }
                _ => i += 1,
            }
        }
        panic!("scan_string: \" not matched!");
    }

    pub fn get_next_token(&mut self) -> Token {
        if self.next_token_info != TokenType::None {
            self.next_token_info = TokenType::None;
            return self.next_token.clone();
        }

        if self.ptr >= self.source[self.line].len() {
            self.line += 1;
            while self.source[self.line].is_empty() {
                self.line += 1;
                if self.line >= self.source.len() {
                    return Token(TokenType::Eof, self.line, None);
                }
            }
            self.ptr = 0;
        }

        let mut current_line = self.source[self.line][self.ptr..].chars();
        self.ptr += 1;
        let c = current_line.next().unwrap();
        match c {
            '$' => Token(TokenType::VarPrefix, self.line, None),

            '(' => Token(TokenType::LeftBracket, self.line, None),

            ')' => Token(TokenType::RightBracket, self.line, None),

            '=' => Token(TokenType::Equal, self.line, None),
            '"' => {
                let (i, s) = self.scan_string(&self.source[self.line], self.ptr);
                self.ptr += i;
                Token(TokenType::String, self.line, Some(s))
            }
            '_' | 'A'..='z' => {
                let (i, s) = self.scan_name(&self.source[self.line], self.ptr - 1);
                self.ptr += i - 1;
                if let Some(token_type) = self.key_words.get(&s) {
                    Token(*token_type, self.line, Some(s))
                } else {
                    Token(TokenType::Name, self.line, Some(s))
                }
            }
            ' ' | '\n' | '\r' | '\t' => Token(TokenType::Ignored, self.line, None),
            _ => panic!(
                "MatchToken(): unexpected symbol {} {}:{} | {}",
                c,
                self.line,
                self.ptr,
                self.source[self.line].len()
            ),
        }
    }

    pub fn next_token_is(&mut self, token_type: TokenType) -> Token {
        let now_type = self.get_next_token();
        if token_type != now_type.0 {
            panic!(
                "next_tokenIs(): expect {:?} found {:?}",
                token_type, now_type
            );
        }
        now_type
    }

    pub fn lookahead(&mut self) -> Token {
        let now_type = self.get_next_token();
        self.next_token = now_type.clone();
        self.next_token_info = now_type.0;
        now_type
    }
}
