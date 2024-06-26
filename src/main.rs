
//!
//! Project:        weather_gov
//!
//! Description:    Uses the weather_gov api.
//!
//!                 For stations given in a config file, periodically
//!                 get the latest observation, and store in a local
//!                 mysql database.
//!                 It assumes the database weather_gov exists.
//!
//!
//! Outline:
//!
//!     1.  Parse yml config, weather_gov.yml.
//!     2.  Create the local database tables.
//!     3.  Get station list from config.
//!     4.  For each station, periodically get observations from weather.gov.
//!
//! Running:
//!
//!     Use the build.sh script from top level like this: ./build.sh run.
//!     Cargo run in the top level dir will not work, but will work from the src dir.
//!     This is because weather_gov.yml needs to be in the current directory with the
//!     executable
//!
//!
//! Primary Crates Used:
//!
//!     1. serde_yaml, serde_json.
//!     2. reqwest.
//!     3. colog.
//!     4. sqlx
//!
//!
use std::collections::HashMap;
use std::{thread, time};

// task allows main to not be an async function
use async_std::task;

use log::{error, warn, info, debug};
mod config;
mod station;
mod db;
use chrono::prelude::{DateTime, Utc};
use std::time::SystemTime;

/// main::iso8601.
///  converts a SystemTime to a iso 8601 string
///
/// # Arguments
///*'st' - SystemTime
///
/// # Return
///
/// ISO 8601 string
fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
    // formats like "2001-07-08T00:34:60.026490+09:30"
}

/// main.
///  orchestrates the program flow
///
/// # Arguments
/// None
///
/// # Return
///
/// None
fn main() {
    // Start logging
    colog::init();
    info!("Starting \
           weather_gov");

    // Get the config
    let config = config::Config::get_config();
    info!("YAML config: {:?}", config);

    // Get the stations url for the stations later on
    let host: HashMap<String, String> = config.host_section;
    debug!("Host config: {:?}", host);
    let stations_url = match host.get("STATIONS_URL") {
        Some(url) => url,
        _ => panic!("Config is missing STATIONS_URL."),
    };

    // Get the polling interval - how often to get station
    //                            observations
    let parms: HashMap<String, String> = config.parameters_section;
    let interval = match parms.get("OBS_INTERVAL_SECS") {
        Some(inter) => inter,
        None  => "300",
    };

    let obs_interval = match interval.parse::<u64>()  {
        Ok(o) => o,
        Err(_) => 300,
    };
    info!("obs_interval: {:?}", obs_interval);

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

    // Need to crank up our db here
    let db_sect: HashMap<String, String> = config.db_section;
    debug!("Host config: {:?}", db_sect);
    let db = &mut db::Db::new(db_sect);
    let res = task::block_on(db.create_tables());
    match res {
        Ok(r) => r,
        Err(err) => panic!("Fatal: could not create database tables: {:?}", err),
    };


    // Get the station json meta data
    let mut station_iter = station_list.iter_mut();
    for station in &mut station_iter {
        let res = task::block_on(station.get_station_json());
        let json = match res {
            Ok(r) => r,
            Err(err) => panic!("Could not get station json {:?}", err),
        };
        debug!("Returned Station json: {}", json);

        // Get the station record and add to database if not already there
        let station_record = station.get_station_record();
        info!("Station record: {:?}", station_record);
        let res = task::block_on(db.put_station_record(station_record));
        info!("Put station record result: {:?}", res);

    }


    // We need a new station list iterator here, we consumed the earlier one
    // We need to go through our stations, get latest observation, and put in db
    let mut i: u64 = 0;
    let interval = time::Duration::from_secs(obs_interval);
    loop {
        let now = SystemTime::now();
        let t8601 = iso8601(&now);
        info!("\n\nLOOP ITERATION: {}  TIME: {}", i, t8601);
        i+=1;
        let mut station_iter = station_list.iter_mut();
        for station in &mut station_iter {
            info!("\nGETTING STATION OBSERVATION FOR {:?}, {:?}, latitude {:?}  \
                  longitude {:?}, elevation {:?} meters", station.station_identifier, 
                  station.station_name, station.latitude, station.longitude,
                  station.elevation_meters);
            let res = task::block_on(station.get_latest_observation_data());
            let obs = match res {
                Ok(r) => r,
                Err(e) => { warn!("Failed getting latest observation for station \
                                  {:?}, {:?}: {:?}", station.station_identifier, 
                                  station.station_name, e);
                            continue;
                          }
            };

            info!("Returned observation json for station: {:?}, {:?}: {:?}",
                      station.station_identifier, station.station_name, obs);
            let res = task::block_on(db.put_observation_record(obs));
            match res {
                Ok(r) => {
                    if r.contains("Duplicate") {
                        info!("Put observation record result for station {:?}, {:?}: {:?}",
                              station.station_identifier, station.station_name, &r);
                        info!("Ignoring Duplicate Observation Record for station {:?}, {:?}",
                              station.station_identifier, station.station_name);
                    } else {
                        info!("Put observation record result for station {:?}, {:?}: {:?}",
                              station.station_identifier, station.station_name, r);
                    }
                },
                Err(err) => { error!("Error putting latest observation from station \
                                     {:?}, {:?}: {:?}", station.station_identifier, 
                                     station.station_name, err); }
            }
        }

        thread::sleep(interval);
    }
}
