use clap::ArgMatches;

pub struct Config {
    pub file: String
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Config {
        let file = matches.get_one("file").cloned().unwrap();

        Config {
            file
        }
    }
}
