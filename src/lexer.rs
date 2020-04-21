
use regex::Regex;

use std::str::CharIndices;
use std::str::FromStr;

use std::collections::HashMap;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Clone, Debug)]
pub enum Tok {
    Newline,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Comma,
    Dot,
    SingleEq,
    Rarrow,
    Mult,
    Divide,
    Plus,
    Minus,
    Greater,
    Lesser,
    Neq,
    Leq,
    Geq,
    DoubleEq,
    Not,
    Or,
    And,
    Identifier(String),
    Int(i64),
    Float(f64),
    StringLit(String),
    If,
    Then,
    Else,
    Elif,
    For,
    In,
    While,
    Case,
    Of,
    Func,
    Return,
}

pub fn get_keywords() -> HashMap<String, Tok> {
    let mut res = HashMap::new();
    
    res.insert("if".to_string(), Tok::If);
    res.insert("then".to_string(), Tok::Then);
    res.insert("else".to_string(), Tok::Else);
    res.insert("elif".to_string(), Tok::Elif);
    res.insert("for".to_string(), Tok::For);
    res.insert("in".to_string(), Tok::In);
    res.insert("while".to_string(), Tok::While);
    res.insert("case".to_string(), Tok::Case);
    res.insert("of".to_string(), Tok::Of);
    res.insert("func".to_string(), Tok::Func);
    res.insert("return".to_string(), Tok::Return);

    res
}

enum ParsedNum {
    Int(i64),
    Float(f64),
}

pub struct Lexer<'input> {
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
        }
    }

    fn parse_string_lit(&mut self) -> Result<(String, usize), ()> {
        let mut res: String = String::new();
        loop {
            match self.chars.next() {
                None => return Err(()),
                Some((_, '\\')) => {
                    // Escaped character
                    match self.chars.next() {
                        None => return Err(()),
                        Some((_, 'n')) => res.push('\n'),
                        Some((_, 't')) => res.push('\t'),
                        Some((_, '"')) => res.push('"'),
                        Some((_, '\\')) => res.push('\\'),
                        _ => return Err(()),
                    }
                },
                Some((i, '"')) => return Ok((res, i)),
                Some((_, c)) => {
                    res.push(c);
                },
            }
        }
    }

    fn parse_ident(&mut self, initial: char, initial_index: usize) -> (String, usize) {
        let mut result = String::new();
        result.push(initial);
        let mut curr_index = initial_index;
        loop {
            match self.chars.peek() {
                None => {return (result, curr_index);},
                Some((i, c)) => {
                    curr_index = *i;
                    if c.is_alphanumeric() || *c == '_' {
                        result.push(*c);
                        self.chars.next();
                    } else {
                        return (result, *i);
                    }
                },
            }
        }
    }

    fn parse_num(&mut self, initial: char, initial_index: usize) -> Result<(ParsedNum, usize), ()> {
        let mut result = String::new();
        result.push(initial);
        let mut curr_index = initial_index;
        let mut found_decimal = false;

        loop {
            match self.chars.peek() {
                None => {
                    if found_decimal {
                        return Ok((ParsedNum::Float(f64::from_str(result.as_ref()).unwrap()), curr_index));
                    } else {
                        return Ok((ParsedNum::Int(i64::from_str(result.as_ref()).unwrap()), curr_index));
                    }
                },
                Some((i, c)) => {
                    curr_index = *i;
                    if found_decimal && *c == '.' {
                        return Err(());
                    }
                    if *c == '.' {
                        found_decimal = true;
                        result.push('.');
                        self.chars.next();
                        continue;
                    }
                    if c.is_ascii_digit() {
                        result.push(*c);
                        self.chars.next();
                        continue;
                    } else if *c == '_' {
                        self.chars.next();
                        continue;
                    }
                    if found_decimal {
                        return Ok((ParsedNum::Float(f64::from_str(result.as_ref()).unwrap()),*i));
                    } else {
                        return Ok((ParsedNum::Int(i64::from_str(result.as_ref()).unwrap()), *i));
                    }
                }
            }
        }
    }

    fn comment_line(&mut self) {
        loop {
            let next = self.chars.next();
            if next == None {
                return;
            }
            if let Some((_, c)) = next {
                if c == '\n' {
                    return;
                }
            }
        }
    }

    fn eat_blank_lines(&mut self) {
        loop {
            match self.chars.peek() {
                Some((_, ' ')) | Some((_, '\t')) | Some((_, '\n')) => {self.chars.next();},
                _ => return,
            }
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok, usize, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            dbg!(&self.chars.peek());
            match self.chars.next() {
                Some((_, ' ')) | Some((_, '\t')) => continue,
                Some((i, '\n')) => {
                    self.eat_blank_lines();
                    return Some(Ok((i, Tok::Newline, i + 1)))
                },
                Some((i, '(')) => return Some(Ok((i, Tok::OpenParen, i + 1))),
                Some((i, ')')) => return Some(Ok((i, Tok::CloseParen, i + 1))),
                Some((i, '[')) => return Some(Ok((i, Tok::OpenBracket, i + 1))),
                Some((i, ']')) => return Some(Ok((i, Tok::CloseBracket, i + 1))),
                Some((i, '{')) => return Some(Ok((i, Tok::OpenBrace, i + 1))),
                Some((i, '}')) => return Some(Ok((i, Tok::CloseBrace, i + 1))),
                Some((i, ',')) => return Some(Ok((i, Tok::Comma, i + 1))),
                Some((i, '.')) => return Some(Ok((i, Tok::Dot, i + 1))),
                Some((i, '*')) => return Some(Ok((i, Tok::Mult, i + 1))),
                Some((i, '/')) => return Some(Ok((i, Tok::Divide, i + 1))),
                Some((i, '+')) => return Some(Ok((i, Tok::Plus, i + 1))),
                Some((i, '-')) => return Some(Ok((i, Tok::Minus, i + 1))),
                Some((i, '!')) => return Some(Ok((i, Tok::Not, i + 1))),
                Some((i, '=')) => {
                    match self.chars.peek() {
                        Some((_, '>')) => {
                            self.chars.next();
                            return Some(Ok((i, Tok::Rarrow, i + 2)));
                        },
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((i, Tok::DoubleEq, i + 2)));
                        },
                        _ => {
                            return Some(Ok((i, Tok::SingleEq, i + 1)));
                        },
                    }
                },
                Some((i, '<')) => {
                    match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((i, Tok::Leq, i + 2)));
                        },
                        _ => {
                            return Some(Ok((i, Tok::Lesser, i + 1)));
                        },
                    }
                },
                Some((i, '>')) => {
                    match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((i, Tok::Geq, i + 2)));
                        },
                        _ => {
                            return Some(Ok((i, Tok::Greater, i + 1)));
                        },
                    }
                },
                Some((i, '!')) => {
                    match self.chars.peek() {
                        Some((_, '=')) => {
                            self.chars.next();
                            return Some(Ok((i, Tok::Neq, i + 2)));
                        },
                        _ => {
                            return Some(Ok((i, Tok::Not, i + 1)));
                        },
                    }
                },
                Some((i, '|')) => {
                    match self.chars.peek() {
                        Some((_, '|')) => {
                            self.chars.next();
                            return Some(Ok((i, Tok::Or, i + 2)));
                        },
                        _ => {
                            return Some(Err(()));
                        },
                    }
                },
                Some((i, '&')) => {
                    match self.chars.peek() {
                        Some((_, '&')) => {
                            self.chars.next();
                            return Some(Ok((i, Tok::And, i + 2)));
                        },
                        _ => {
                            return Some(Err(()));
                        },
                    }
                },
                Some((i, '"')) => {
                    let lit = self.parse_string_lit();
                    if let Err(_) = lit {
                        return Some(Err(()));
                    }
                    let (s, i2) = lit.unwrap();
                    return Some(Ok((i, Tok::StringLit(s), i2)));
                },
                Some((i, '#')) => {
                    self.comment_line();
                },
                Some((i, c)) => {
                    if c.is_alphabetic() {
                        let (s, i2) = self.parse_ident(c, i);
                        // Check if the indentifier was a keyword
                        let keywords = get_keywords();
                        if let Some(tok) = keywords.get(&s) {
                            return Some(Ok((i, tok.clone(), i2)));
                        }
                        return Some(Ok((i, Tok::Identifier(s), i2)));
                    } else if c.is_ascii_digit() {
                        let val = self.parse_num(c, i);
                        match val {
                            Ok((ParsedNum::Int(val), i2)) => return Some(Ok((i, Tok::Int(val), i2))),
                            Ok((ParsedNum::Float(val), i2)) => return Some(Ok((i, Tok::Float(val), i2))),
                            Err(x) => return Some(Err(x)),
                        }
                    }
                },
                None => return None, // EOF
            }
        }
    }
}
