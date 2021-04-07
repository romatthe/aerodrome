use crate::settings::Settings;
use std::error::Error;

mod settings;



fn main() -> Result<(), Box<dyn Error>>{
    env_logger::init();
    log::info!("Starting up!");

    let settings = Settings::init()?;
    log::debug!("Loading configuration: {:?}", settings);

    Ok(())
}
