
use std::collections::HashMap;
use mysql::prelude::*;


pub struct Db {
    pub host:              String,
    pub user:              String,
    pub password:          String,
    pub database:          String,
    pub station_table:     String,
    pub observation_table: String,

}


impl Db {

    pub fn new(cfg: HashMap<String, String>) -> Db {

        Self {
            host:               cfg["host"].clone(),
            user:               cfg["user"].clone(),
            password:           cfg["password"].clone(),
            database:           cfg["database"].clone(),
            station_table:      cfg["station_table"].clone(),
            observation_table:  cfg["observation_table"].clone(),
        }
    }

    pub fn create_tables(&mut self) {

        let pool =
            mysql::Pool::new("mysql://weather_user:weather_pass@localhost:3306/weather_gov")
            .unwrap();

        let mut conn = pool.get_conn().expect("Failed to get a connection from the pool");

        conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS station_rust (call_id VARCHAR(5) PRIMARY KEY,
        name VARCHAR(80), latitude_deg FLOAT, longitude_deg FLOAT, elevation_m FLOAT,
        url VARCHAR(80))"
        ).expect("Failed to create station table");

        println!("Station Table created successfully!");

        conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS observation_rust (station_id VARCHAR(20),
        timestamp_UTC VARCHAR(40), temperature_C FLOAT, temperature_F FLOAT,
        dewpoint_C FLOAT, dewpoint_F FLOAT, description VARCHAR(40), wind_dir FLOAT,
        wind_spd_km_h FLOAT, wind_spd_mi_h FLOAT, wind_gust_km_h FLOAT,
        wind_gust_mi_h FLOAT, baro_pres_pa FLOAT, baro_pres_inHg FLOAT,
        rel_humidity FLOAT, PRIMARY KEY (station_id, timestamp_UTC))"
        ).expect("Failed to create observation table");

        println!("Observation Table created successfully!");
    }

} // impl Db
