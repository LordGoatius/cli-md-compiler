pub mod app;
pub mod config;
pub mod lexer;
pub mod parser;

use crate::config::Config;

use clap::{
    crate_description, crate_name, crate_version, Arg,
    ArgAction::{Append, Help},
    Command,
};

fn main() {
    let args = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .arg(
            Arg::new("file")
                .help("Must supply a file")
                .action(Append)
        )
        .disable_help_flag(true)
        .arg(
            Arg::new("help")
                .short('H')
                .long("help")
                .action(Help)
                .help("Prints help information")
        )
        .get_matches();

    let conf = Config::new(&args);

    app::run(conf);
}
