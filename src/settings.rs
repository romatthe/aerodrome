use config::{ConfigError, Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Folders {
    data_folder: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    folders: Folders,
}

impl Settings {
    pub fn init() -> Result<Self, ConfigError> {
        let mut c = Config::new();

        // Load the configuration from the default location
        c.merge(File::with_name("aerodrome"))?;

        c.try_into()
    }
}