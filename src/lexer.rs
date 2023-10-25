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
    condense_lex(token_string)
}

fn condense_lex(mut token_string: Vec<Token>) -> Vec<Token> {
    let _tokn_comparison_vector: Vec<Vec<Token>> = 
    vec![vec![Token::Code, Token::Code, Token::Code], // Code Block
         vec![Token::Asterisk, Token::Asterisk],
         vec![Token::Underscore, Token::Underscore],
         vec![Token::Exclamation, Token::BracketOpen]];

     for (i, win) in token_string.clone().windows(3).enumerate() {
         if win[0] == Token::Code &&
            win[1] == Token::Code &&
            win[2] == Token::Code {
             token_string[i] = Token::CodeBlock;
             token_string[i + 1] = Token::Remove;
             token_string[i + 2] = Token::Remove;
         }
         println!("{i}: {win:?}");
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
