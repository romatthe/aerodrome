use std::error::Error;
use aerodrome_core::settings::Settings;
use aerodrome_core::app::AerodromeApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    // env_logger::init();
    log::info!("Starting up!");

    let settings = Settings::init()?;
    log::debug!("Loading configuration: {:?}", settings);

    // TODO: Pass in configuration
    let app = AerodromeApp::init().await;

    app.run().await;

    Ok(())
}
