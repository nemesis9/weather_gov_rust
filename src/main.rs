
// Project:        weather_gov
// Description:    for given stations, get observations
//
// Outline:
//     1.  Parse yaml config
//     2.  create db
//     3.  Get station list from config
//     4.  For each station, periodically get observations from weather.gov
//

use log::{error, warn, info, debug};
mod config;



fn main() {
    colog::init();
    //println!("Welcome to weather_gov");
    //error!("Starting weather_gov");
    //warn!("Starting weather_gov");
    info!("Starting weather_gov");
    //debug!("Starting weather_gov");

    let config = match config::get_config() {
        Ok(_result) => _result,
        Err(error) => panic!("Error: {:?}", error),
    };


    info!("YAML config: {:?}", config);

}
