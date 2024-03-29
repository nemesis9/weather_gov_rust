use log::{error, warn, info, debug};

use serde::{Deserialize, Serialize};
use serde_yaml::{self};

use std::collections::HashMap;
// Project:        weather_gov
// Description:    for given stations, get observations
//
// Outline:
//     1.  Parse yaml config
//     2.  Get station list from config
//     3.  For each station, periodically get observations from weather.gov
//
//
#[derive(Debug, Serialize, Deserialize)]
struct Config {
   log_section:        HashMap<String, String>,
   host_section:       HashMap<String, String>,
   db_section:         HashMap<String, String>,
   stations_section:   HashMap<String, String>,
   parameters_section: HashMap<String, String>,
}


fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let _f = std::fs::File::open("./weather_gov.yml").expect("Could not open yml config");
    let _c: Config = serde_yaml::from_reader(_f).expect("Could not create config from yml");
    //println!("Read YAML config: {:?}", _c);
    Ok(_c)
}

fn main() {
    colog::init();
    //println!("Welcome to weather_gov");
    //error!("Starting weather_gov");
    //warn!("Starting weather_gov");
    info!("Starting weather_gov");
    //debug!("Starting weather_gov");

    let config = match get_config() {
        Ok(_result) => _result,
        Err(error) => panic!("Error: {:?}", error),
    };


    info!("YAML config: {:?}", config);

}
