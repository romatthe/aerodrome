use crate::settings::Settings;
use std::error::Error;
use crate::app::AerodromeApp;

mod app;
mod settings;
mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    env_logger::init();
    log::info!("Starting up!");

    let settings = Settings::init()?;
    log::debug!("Loading configuration: {:?}", settings);

    let app = AerodromeApp::new();

    app.run().await;

    Ok(())
}
