use clap::ArgMatches;

#[derive(Debug, Clone)]
pub struct Config {
    pub verbose: bool,
    pub dir: String,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Self, String> {
        let verbose = matches.is_present("verbose");
        match matches.value_of("dir") {
            Some(dir) => Ok(Config { verbose, dir: dir.to_string() }),
            None => Result::Err("Arg dir not passed".to_string()),
        }
    }
}
