
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
   pub log_section:        HashMap<String, String>,
   pub host_section:       HashMap<String, String>,
   pub db_section:         HashMap<String, String>,
   pub stations_section:   HashMap<String, String>,
   pub parameters_section: HashMap<String, String>,
}

impl Config {

    //pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    //
    // We must panic if we cannot get the config -> the file exists and is valid yaml
    pub fn get_config() -> Config {
        let _f = std::fs::File::open("./weather_gov.yml").expect("Could not open yml config. \
               Please create and use a valid weather_gov.yml.");
        let _c: Config = serde_yaml::from_reader(_f).expect("Could not create config from yml. \
               Please validate the yaml config file.");

        Self {
              log_section: _c.log_section,
              host_section: _c.host_section,
              db_section: _c.db_section,
              stations_section: _c.stations_section,
              parameters_section: _c.parameters_section,
        }
    }


} // impl Config
