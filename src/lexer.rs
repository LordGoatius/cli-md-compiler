use std::{fmt::{self, Display}, fs};

#[derive(Clone)]
enum Token<'a> {
    Text(&'a str),
    Newline,
    Header(HeaderValue),
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
    Backslash
}

#[derive(Copy, Clone)]
enum HeaderValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderValue::One => {
                write!(f, "#")
            },
            HeaderValue::Two => {
                write!(f, "##")
            },
            HeaderValue::Three => {
                write!(f, "###")
            },
            HeaderValue::Four => {
                write!(f, "####")
            },
            HeaderValue::Five => {
                write!(f, "#####")
            },
            HeaderValue::Six => {
                write!(f, "######")
            },
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Text(value) => {
                write!(f, "{value}")
            },
            Token::Newline => {
                write!(f, "\n")
            },
            Token::Header(header_value) => {
                write!(f, "{header_value}")
            },
            Token::Bold => {
                write!(f, "**")
            },
            Token::Italic => {
                write!(f, "__")
            },
            Token::Blockquote => {
                write!(f, ">")
            },
            Token::UnorderedList => {
                write!(f, "-")
            },
            Token::OrderedList => {
                write!(f, "1)")
            },
            Token::Code => {
                write!(f, "`")
            },
            Token::CodeBlock => {
                write!(f, "```")
            },
            Token::BracketOpen => {
                write!(f, "[")
            },
            Token::BracketClose => {
                write!(f, "]")
            },
            Token::ParenthesisOpen => {
                write!(f, "(")
            },
            Token::ParenthesisClose => {
                write!(f, ")")
            },
            Token::AngleBracketOpen => {
                write!(f, "<")
            },
            Token::AngleBracketClose => {
                write!(f, ">")
            },
            Token::BraceOpen => {
                write!(f, "{{")
            },
            Token::BraceClose => {
                write!(f, "}}")
            },
            Token::ImageOpen => {
                write!(f, "![")
            },
            Token::ImageClose => {
                write!(f, "!]")
            },
            Token::Backslash => {
                write!(f, "\\")
            },
        }
    }
}

pub fn lex(file_path: String, output_path: String) {
    let mut token_string: Vec<Token>;
    let file_contents = fs::read(file_path).expect("Failed to read file");

}
