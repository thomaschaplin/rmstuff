use async_std::task;

mod analysis;
mod config;
mod detectors;
mod error;
mod size;

fn main() -> Result<(), error::RmStuffError> {
    let conf = config::init_config();

    task::block_on(analysis::scheduler(conf))?;

    Ok(())
}
