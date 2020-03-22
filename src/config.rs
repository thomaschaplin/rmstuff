use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    name = "rmstuff",
    version = "1.0",
    author = "Stjepan Golemac <stjepan.golemac@gmail.com",
    about = "Removes all unecessary files from projects"
)]
pub struct Config {
    #[structopt(short, long)]
    pub verbose: bool,
    #[structopt(short, long)]
    pub dry_run: bool,
    pub dir: String,
}

pub fn init_config() -> Config {
    Config::from_args()
}
