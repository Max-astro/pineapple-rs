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
    pub nextTokenInfo: TokenType,
    nextToken: Token,
    keyWords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut hash = HashMap::new();
        hash.insert("print".to_string(), TokenType::Print);
        Lexer {
            source: source.split('\n').map(|s| s.to_string()).collect(),
            line: 0,
            ptr: 0,
            nextTokenInfo: TokenType::None,
            nextToken: Token(TokenType::None, 0, None),
            keyWords: hash,
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.source);
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
        if self.nextTokenInfo != TokenType::None {
            self.nextTokenInfo = TokenType::None;
            return self.nextToken.clone();
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
            '$' => return Token(TokenType::VarPrefix, self.line, None),

            '(' => return Token(TokenType::LeftBracket, self.line, None),

            ')' => return Token(TokenType::RightBracket, self.line, None),

            '=' => return Token(TokenType::Equal, self.line, None),
            '"' => {
                let (i, s) = self.scan_string(&self.source[self.line], self.ptr);
                self.ptr += i;
                return Token(TokenType::String, self.line, Some(s));
            }
            '_' | 'A'..='z' => {
                let (i, s) = self.scan_name(&self.source[self.line], self.ptr - 1);
                self.ptr += i - 1;
                if let Some(token_type) = self.keyWords.get(&s) {
                    return Token(token_type.clone(), self.line, Some(s));
                } else {
                    return Token(TokenType::Name, self.line, Some(s));
                }
            }
            ' ' | '\n' | '\r' | '\t' => return Token(TokenType::Ignored, self.line, None),
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
                "NextTokenIs(): expect {:?} found {:?}",
                token_type, now_type
            );
        }
        now_type
    }

    pub fn lookahead(&mut self) -> Token {
        let now_type = self.get_next_token();
        self.nextToken = now_type.clone();
        self.nextTokenInfo = now_type.0;
        now_type
    }
}
