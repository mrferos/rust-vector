use std::env;
use config::{Config, File, ConfigError};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Udp {
    #[serde(default="get_default_udp_host")]
    host: String,
    port: String,
    #[serde(default="get_default_udp_byte_size")]
    byte_size: i32,
}
#[derive(Deserialize, Debug)]
pub struct Settings {
    udp: Udp
}
fn get_default_udp_host() -> String {
    return "127.0.0.1".to_string()
}
fn get_default_udp_byte_size() -> i32 {
    return 8 * 1024;
}

impl Udp {
    pub fn host(&self) -> &str {
        return &self.host
    }

    pub fn port(&self) -> &str {
        return &self.port
    }

    pub fn byte_size(&self) -> i32 {
        return self.byte_size
    }
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "local".into());

        let s = Config::builder()
            .add_source(
                File::with_name(&format!("config/{}.yaml", run_mode))
            )
            .build()?;

        return s.try_deserialize()
    }

    pub fn udp(&self) -> &Udp {
        return &self.udp
    }
}