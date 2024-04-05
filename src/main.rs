
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

use log::{info, debug};
mod config;
mod station;
mod db;


fn main() {
    // Start logging
    colog::init();
    info!("Starting weather_gov");

    // Get the config
    let config = config::Config::get_config();
    info!("YAML config: {:?}", config);

    let host: HashMap<String, String> = config.host_section;
    debug!("Host config: {:?}", host);
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

        debug!("Returned Station json: {}", json);
        info!("Station json in station object: {}", station.json_station_data);

        station.set_station_data();
    }

    // Need to crank up our db here
    let db_sect: HashMap<String, String> = config.db_section;
    debug!("Host config: {:?}", db_sect);
    let db = &mut db::Db::new(db_sect);
    let res = task::block_on(db.create_tables());
    match res {
        Ok(r) => r,
        Err(err) => panic!("Could not create database tables: {:?}", err),
    };

    // We need a new station list iterator here, we consumed the earlier one
    let mut station_iter = station_list.iter_mut();
    for station in &mut station_iter {
        let res = task::block_on(station.get_latest_observation_data());
        let json = match res {
            Ok(r) => r,
            Err(e) => format!("Bad Request {:?}", e),
        };

        info!("Returned observation json: {}", json);
    }
}
