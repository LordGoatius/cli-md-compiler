use std::fs;

use crate::config::Config;
use crate::lexer;
use crate::parser;

pub fn run(config: Config) {
    let file = fs::read_to_string(config.file).expect("Failed to read");
    let lines = file.lines();
    let token_string = lexer::lex(lines);

    for tok in token_string.clone() {
        println!("{tok:?}");
    }
    for tok in &token_string {
        print!("{tok}");
    }
    let _output_string = parser::parse(token_string);
}
