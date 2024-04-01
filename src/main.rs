
// Project:        weather_gov
// Description:    for given stations, get observations
//
// Outline:
//     1.  Parse yaml config
//     2.  create db
//     3.  Get station list from config
//     4.  For each station, periodically get observations from weather.gov
//
use std::collections::HashMap;
// task allows main to not be an async function
use async_std::task;

use log::{error, warn, info, debug};
mod config;
mod station;


fn main() {
    // Start logging
    colog::init();
    info!("Starting weather_gov");

    // Get the config
    let config = match config::get_config() {
        Ok(_result) => _result,
        Err(error) => panic!("Error: {:?}", error),
    };
    info!("YAML config: {:?}", config);

    let host: HashMap<String, String> = config.host_section;
    debug!("Host config: {:?}", host);
    //let stations_url: <String> = host["STATIONS_URL"];
    let stations_url = match host.get("STATIONS_URL") {
        Some(url) => url,
        _ => panic!("Config is missing STATIONS_URL."),
    };

    // Get the stations from the config
    let stations: HashMap<String, String> = config.stations_section;
    info!("Stations config: {:?}", stations);

    // Create station objects and add to station list
    let mut station_list = Vec::<station::Station>::new();
    for (key, value) in stations {
        debug!("{} / {}", key, value);
        let station = station::Station::new(String::from(value),
                                            String::from(stations_url));
        station_list.push(station);
    }

    // Get the station json meta data
    let mut station_iter = station_list.iter_mut();
    for station in &mut station_iter {
        info!("Station id {}, station_url: {}", station.station_identifier,
                                                   station.station_url);
        info!("Station observation url: {}", station.observation_url);
        let res = task::block_on(station.get_station_json());
        let json = match res {
            Ok(r) => r,
            Err(err) => panic!("Could not get station json {:?}", err),
        };

        info!("Returned Station json: {}", json);
        info!("Station json in station object: {}", station.json_data);

        let res = station.parse_json(&json);
        match res {
            Ok(e) => e,
            Err(e) => { println!("Err: {:?}", e); },
        }

    }

}
