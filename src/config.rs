// src/config.rs

use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Browser {
    Chrome,
    Edge,
    Firefox,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub application_url: String,
    pub wait_time: u64,
    pub domain: String,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postcode: String,
    pub args: Vec<String>,
    pub browser: Browser,
    pub webdriver_url: String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::de::from_str(&contents)?;
        Ok(config)
    }
}
