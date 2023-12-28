use std::io::{self, Write};

use crate::lexer::Token;
use owo_colors::{OwoColorize, Style, colors::CustomColor};

struct ParserState {
    bold: bool,
    italic: bool,
    code: bool,
    code_block: bool,
}

impl ParserState {
    fn toggle_bold(&mut self) {
        self.bold = !self.bold;
    }

    fn toggle_italic(&mut self) {
        self.italic = !self.italic;
    }

    fn toggle_code(&mut self) {
        self.code = !self.code;
    }

    fn toggle_codeblock(&mut self) {
        self.code_block = !self.code_block;
    }

    fn to_style(&self) -> Style {
        let mut style = Style::new();
        if self.bold {
            style = style.bold()
        } 
        if self.italic {
            style = style.italic()
        }
        if self.code {
            style = style.fg::<CustomColor<0x45, 0x45, 0x45>>();
        }

        style
    }
}

impl Default for ParserState {
    fn default() -> Self {
        ParserState { bold: false, italic: false, code: false, code_block: false }
    }
}

pub fn parse(token_string: Vec<Token>) {
    let mut state = ParserState::default();
    for line in token_string.split(|token| *token == Token::Newline) {
        for token in line {
            // if token not text/printed, update state
            if *token == Token::Bold {
                state.toggle_bold();
            }
            if *token == Token::Italic {
                state.toggle_italic();
            }
            if *token == Token::Code {
                state.toggle_code()
            }
            if *token == Token::CodeBlock {
                state.toggle_codeblock()
            }

            if let Token::Text(text) = token {
                let print_style = state.to_style();
                print!("{}", text.style(print_style));
                io::stdout().flush().unwrap();
            }
        }
        println!();
    }
}
