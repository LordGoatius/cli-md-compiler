use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Text(String),
    Asterisk,
    Underscore,
    Newline,
    Header,
    Bold,
    Italic,
    Blockquote,
    UnorderedList,
    OrderedList,
    Code,
    CodeBlock,
    BracketOpen,
    BracketClose,
    ParenthesisOpen,
    ParenthesisClose,
    AngleBracketOpen,
    AngleBracketClose,
    BraceOpen,
    BraceClose,
    ImageOpen,
    ImageClose,
    Backslash,
    Exclamation,
    Remove
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Text(value) => write!(f, "{value}"),
            Token::Newline => write!(f, "\n"),
            Token::Header => write!(f, "#"),
            Token::Asterisk => write!(f, "*"),
            Token::Underscore => write!(f, "_"),
            Token::Bold => write!(f, "**"),
            Token::Italic => write!(f, "__"),
            Token::Blockquote => write!(f, ">"),
            Token::UnorderedList => write!(f, "-"),
            Token::OrderedList => write!(f, "1)"),
            Token::Code => write!(f, "`"),
            Token::CodeBlock => write!(f, "```"),
            Token::BracketOpen => write!(f, "["),
            Token::BracketClose => write!(f, "]"),
            Token::ParenthesisOpen => write!(f, "("),
            Token::ParenthesisClose => write!(f, ")"),
            Token::AngleBracketOpen => write!(f, "<"),
            Token::AngleBracketClose => write!(f, ">"),
            Token::BraceOpen => write!(f, "{{"),
            Token::BraceClose => write!(f, "}}"),
            Token::ImageOpen => write!(f, "!["),
            Token::ImageClose => write!(f, "!]"),
            Token::Backslash => write!(f, "\\"),
            Token::Exclamation => write!(f, "!"),
            Token::Remove => write!(f, "[[REMOVE]]"),
        }
    }
}

pub fn lex<'a, I>(lines: I) -> Vec<Token>
where 
    I: Iterator<Item = &'a str>
{
    let mut token_string: Vec<Token> = vec![];
    for line in lines {
        token_string.append(&mut lex_line(&line));
    }
    condense_lex_4(condense_lex_3(condense_lex_2(condense_lex_1(token_string))))
}

fn condense_lex_4(token_string: Vec<Token>) -> Vec<Token> {
    let mut return_str: Vec<Token> = vec![];

    let mut text_str: String = String::from("");

    for tok in token_string.iter() {
        if let Token::Text(char) = tok {
            text_str.push_str(char);
        } else {
            if text_str == "" {
                return_str.push(tok.clone());
            } else {
                return_str.push(Token::Text(text_str.clone()));
                return_str.push(tok.clone());
                text_str = String::from("");
            }
        }
    }

    return_str
}

fn condense_lex_3(token_string: Vec<Token>) -> Vec<Token> {
    let mut return_str: Vec<Token> = vec![];
    for item in token_string.iter().filter(|item| **item != Token::Remove) {
        return_str.push(item.clone());
    }
    return_str
}

fn condense_lex_2(mut token_string: Vec<Token>) -> Vec<Token> {
     for (i, win) in token_string.clone().windows(2).enumerate() {
         if win[0] == Token::Asterisk && win[1] == Token::Asterisk {
             token_string[i] = Token::Bold;
             token_string[i + 1] = Token::Remove;
         } else if win[0] == Token::Underscore && win[1] == Token::Underscore {
             token_string[i] = Token::Italic;
             token_string[i + 1] = Token::Remove;
         } else if win[0] == Token::Exclamation && win[1] == Token::BracketOpen {
             token_string[i] = Token::ImageOpen;
             token_string[i + 1] = Token::Remove;
         }
     }

    token_string
}

fn condense_lex_1(mut token_string: Vec<Token>) -> Vec<Token> {
     for (i, win) in token_string.clone().windows(3).enumerate() {
         if win[0] == Token::Code &&
            win[1] == Token::Code &&
            win[2] == Token::Code {
             token_string[i] = Token::CodeBlock;
             token_string[i + 1] = Token::Remove;
             token_string[i + 2] = Token::Remove;
         }
     }

    token_string
}

fn lex_line(line: &str) -> Vec<Token> {
    let mut token_string: Vec<Token> = vec![];
    let char_iter = line.chars();

    let mut temp_str = "".to_owned();

    for char in char_iter {
        match char {
            '#' => token_string.push(Token::Header),
            '-' => token_string.push(Token::UnorderedList),
            '`' => token_string.push(Token::Code),
            '*' => token_string.push(Token::Asterisk),
            '_' => token_string.push(Token::Underscore),
            '>' => token_string.push(Token::Blockquote),
            '[' => token_string.push(Token::BracketOpen),
            ']' => token_string.push(Token::BracketClose),
            '(' => token_string.push(Token::ParenthesisOpen),
            ')' => token_string.push(Token::ParenthesisClose),
            '{' => token_string.push(Token::BraceOpen),
            '}' => token_string.push(Token::BraceClose),
            '!' => token_string.push(Token::Exclamation),
            '\\' => token_string.push(Token::Backslash),
            val => temp_str.push(val),
        }
        match char {
            '#' | '-' | '`' | '*' | '_' | '>' | '[' | 
            ']' | '(' | ')' | '{' | '}' | '!' | '\\' => (),
            _ => {
                token_string.push(Token::Text(temp_str.clone()));
                temp_str = "".to_string();
            }         
        }
    }

    token_string.push(Token::Newline);
    
    token_string
}
