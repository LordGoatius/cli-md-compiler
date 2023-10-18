use crate::config::Config;
use crate::lexer;

pub fn run(config: Config) {
    let file_path = config.file;
    let output_path = "output.txt".to_owned();
    lexer::lex(file_path, output_path);
}
