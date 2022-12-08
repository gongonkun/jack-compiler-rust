use std::{str::Chars, iter::Peekable};

#[derive(Debug)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}
#[derive(Debug)]
pub enum TokenKeyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub keyword_type: Option<TokenKeyword>
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a String) -> Tokenizer<'a> {
        Tokenizer { 
            source: source.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let iter = &mut self.source;

        while let Some(token_char) = iter.next() {
            match token_char {
                ' ' | '\n' | '\r' | '\t' => {
                    continue;
                },

                '/' => {
                    /* Delete Comment */
                    if let Some(next_token_char) = iter.peek() {
                        // // ~ 改行までコメント
                        if *next_token_char == '/' {
                            while let Some(comment_char) = iter.next() {
                                if comment_char == '\n' {
                                    break;
                                }
                            }
                        } else if *next_token_char == '*' {
                            // /* ~  */ までコメント
                            while let Some(comment_char) = iter.next() {
                                if comment_char == '*' {
                                    match iter.next() {
                                        Some(c) if c == '/' => { break; },
                                        None => { break; },
                                        _ => {},
                                    }
                                }
                            }
                        } else {
                            // TODO: "/" はシンボルなのでいい感じに対応する。
                            return Some(Token {
                                value: token_char.to_string(),
                                token_type: TokenType::Symbol,
                                keyword_type: None,
                            });
                        }
                    }
                },
                /*
                * Symbol
                * "/" だけコメントの可能性がある為一つ上で確認される
                */
                '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' |
                '+' | '-' | '*' | '&' | '|' | '<' | '>' | '=' | '~' => {
                    return Some(Token {
                        value: token_char.to_string(),
                        token_type: TokenType::Symbol,
                        keyword_type: None,
                    });
                },
                /*
                * IntegerConst
                * 任意の数字列
                * JackにはIntegerしか存在しない為、最初に0が来た場合その時点でトークンと見做す
                */
                '0' => {
                    return Some(Token {
                        value: '0'.to_string(),
                        token_type: TokenType::IntConst,
                        keyword_type: None,
                    });
                },
                '1'..='9' => {
                    let mut int_token = token_char.to_string();

                    for integer_const_char in self.source.peek().cloned() {
                        match integer_const_char {
                            '0'..='9' => {
                                int_token += self.source.next().unwrap().to_string().as_str();
                            },
                            _ => {
                                break;
                            }
                        }
                    }

                    return Some(Token {
                        value: int_token,
                        token_type: TokenType::IntConst,
                        keyword_type: None,
                    });
                },
                /*
                * StringConst
                * ダブルクォートで囲われた、改行を含まないユニコードの任意の文字列
                */
                '"' => {
                    let mut str_token = token_char.to_string();

                    while let Some(string_const_char) = self.source.next() {
                        match string_const_char {
                            '"' => {
                                break;
                            },
                            _ => {
                                str_token += string_const_char.to_string().as_str();
                            }
                        }
                    }

                    return Some(Token {
                        value: str_token,
                        token_type: TokenType::StringConst,
                        keyword_type: None,
                    })
                },
                /* 
                * Keyword or Identifier
                * アルファベットから始まる 英数字+'_'はidentifier or keyword
                */
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut keyword_or_identifier = token_char.to_string();

                    while let Some(next_char) = iter.peek().cloned() {
                        match next_char {
                            '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => {
                                keyword_or_identifier += iter.next().unwrap().to_string().as_str();
                            },
                            _ => { break; }
                        }
                    }

                    if let Some(keyword) = Self::keyword_type(&keyword_or_identifier) {
                        return Some(Token {
                            value: keyword_or_identifier,
                            token_type: TokenType::Keyword,
                            keyword_type: Some(keyword),
                        })
                    } else {
                        return Some(Token {
                            value: keyword_or_identifier,
                            token_type: TokenType::Identifier,
                            keyword_type: None,
                        })
                    }
                },
                /*
                * TODO: 不正な文字の時のエラー制御
                */
                _ => { panic!("{} is unexpected character!", token_char) }
            }
        }

        None
    }

    pub fn has_more_token(&mut self) -> bool {
        if let Some(_) = self.source.peek() {
            true
        } else {
            false
        }
    }
    fn keyword_type(token: &str) -> Option<TokenKeyword> {
        match token {
            "class" => Some(TokenKeyword::Class),
            "constructor" => Some(TokenKeyword::Constructor),
            "function" => Some(TokenKeyword::Function),
            "method" => Some(TokenKeyword::Method),
            "field" => Some(TokenKeyword::Field),
            "static" => Some(TokenKeyword::Static),
            "var" => Some(TokenKeyword::Var),
            "int" => Some(TokenKeyword::Int),
            "char" => Some(TokenKeyword::Char),
            "boolean" => Some(TokenKeyword::Boolean),
            "void" => Some(TokenKeyword::Void),
            "true" => Some(TokenKeyword::True),
            "false" => Some(TokenKeyword::False),
            "null" => Some(TokenKeyword::Null),
            "this" => Some(TokenKeyword::This),
            "let" => Some(TokenKeyword::Let),
            "do" => Some(TokenKeyword::Do),
            "if" => Some(TokenKeyword::If),
            "else" => Some(TokenKeyword::Else),
            "while" => Some(TokenKeyword::While),
            "return" => Some(TokenKeyword::Return),
            _ => None,
        }
    }
}

