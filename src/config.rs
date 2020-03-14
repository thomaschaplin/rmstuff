use crate::error;
use clap::ArgMatches;

#[derive(Debug, Clone)]
pub struct Config {
    pub verbose: bool,
    pub dir: String,
}

impl Config {
    pub fn new(matches: ArgMatches) -> error::RmStuffResult<Self> {
        let verbose = matches.is_present("verbose");
        match matches.value_of("dir") {
            Some(dir) => Ok(Config {
                verbose,
                dir: dir.to_string(),
            }),
            None => Result::Err(error::RmStuffError::new("Arg dir not passed")),
        }
    }
}
