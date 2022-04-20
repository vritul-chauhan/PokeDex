use config::{Config, ConfigError, File};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Setting {
    pub SERVER: Server,
}

impl Setting {
    fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("./src/config/settings"))
            .build()?;
        settings.try_deserialize()
    }
}

lazy_static! {
    pub static ref SETTING: Setting = Setting::new().expect("No settings found!");
}
