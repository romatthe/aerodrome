use std::error::Error;
use sqlx::Sqlite;
use aerodrome_core::settings::Settings;
use aerodrome_core::app::AerodromeApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    env_logger::init();
    log::info!("Starting up!");

    let settings = Settings::init()?;
    log::debug!("Loading configuration: {:?}", settings);

    // TODO: Database type should be based on configuration
    let app = AerodromeApp::<Sqlite>::init().await;

    app.run().await;

    Ok(())
}
