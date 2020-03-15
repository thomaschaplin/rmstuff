use {
    async_std::task,
    clap::{App, Arg},
};

mod analysis;
mod config;
mod error;
mod detectors;

fn main() -> Result<(), error::RmStuffError<'static>> {
    let matches = App::new("rmstuff")
        .version("1.0")
        .author("Stjepan Golemac <stjepan.golemac@gmail.com>")
        .about("Removes all unecessary files from projects")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .help("Prints more info"),
        )
        .arg(
            Arg::with_name("dir")
                .help("Sets the dir to search through")
                .required(true)
                .index(1),
        )
        .get_matches();

    let conf = config::Config::new(matches)?;

    task::block_on(analysis::scheduler(conf));

    Ok(())
}
