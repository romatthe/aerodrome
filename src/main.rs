use crate::settings::Settings;
use std::error::Error;
use crate::app::AerodromeApp;
use sqlx::Sqlite;

mod app;
mod model;
mod settings;
mod store;
mod web;

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
