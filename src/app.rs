use std::fs;

use crate::config::Config;
use crate::lexer;

pub fn run(config: Config) {
    let file = fs::read_to_string(config.file).expect("Failed to read");
    let lines = file.lines();
    let token_string = lexer::lex(lines);
    println!("{token_string:?}");
    for tok in token_string {
        print!("{tok}");
    }
}
