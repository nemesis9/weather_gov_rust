
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
   log_section:        HashMap<String, String>,
   host_section:       HashMap<String, String>,
   db_section:         HashMap<String, String>,
   stations_section:   HashMap<String, String>,
   parameters_section: HashMap<String, String>,
}


pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let _f = std::fs::File::open("./weather_gov.yml").expect("Could not open yml config");
    let _c: Config = serde_yaml::from_reader(_f).expect("Could not create config from yml");
    //println!("Read YAML config: {:?}", _c);
    Ok(_c)
}

