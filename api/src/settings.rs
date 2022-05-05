use config::{Config, ConfigError, File};
use serde::Deserialize;

const DEFAULTS_CONFIG_FILE: &str = "config/defaults.toml";
const USER_DEFINED_CONFIG_FILE: &str = "config/settings.toml";

#[derive(Deserialize)]
pub struct InstanceSettings {
    pub port: u16,
}

impl InstanceSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name(DEFAULTS_CONFIG_FILE))
            .add_source(File::with_name(USER_DEFINED_CONFIG_FILE))
            .build()?;
        settings.try_deserialize()
    }
}

pub fn get_instance_settings() -> InstanceSettings {
    match InstanceSettings::new() {
        Ok(settings) => settings,
        Err(error) => panic!("Issue trying to initialise instance settings: {:?}", error),
    }
}
