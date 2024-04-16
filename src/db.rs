
use std::collections::HashMap;
use sqlx::{Pool, MySql, Error, MySqlPool};
use async_std::task;
use crate::station::StationRecord;
use crate::station::ObservationRecord;
use log::{info};

/// Represents a db instance.
pub struct Db {
    pub host:              String,
    pub port:              String,
    pub user:              String,
    pub password:          String,
    pub database:          String,
    pub station_table:     String,
    pub observation_table: String,
    db_pool:           Pool<MySql>,
}


/// Implementation for db instance.
impl Db {

    ///  Creates a new Db instance.
    ///
    /// # Arguments
    ///
    ///*'cfg'-the Db config
    ///
    /// # Return
    ///
    /// Db instance
    pub fn new(cfg: HashMap<String, String>) -> Db  {

        Self {
            host:               cfg["host"].clone(),
            port:               cfg["port"].clone(),
            user:               cfg["user"].clone(),
            password:           cfg["password"].clone(),
            database:           cfg["database"].clone(),
            station_table:      cfg["station_table"].clone(),
            observation_table:  cfg["observation_table"].clone(),

            // Following will crash if we can't connect, but without a db connection there's
            //      not much can be done anyway.
            db_pool:            task::block_on(Db::connect(format!("mysql://{}:{}@{}:{}/{}",
                                cfg["user"], cfg["password"], cfg["host"], cfg["port"],
                                cfg["database"]).as_str())).expect("Could not connect to database"),
        }
    }

    ///  Creates a new Db connection.
    ///
    /// # Arguments
    ///
    ///*'cpath'-the Db url
    ///
    /// # Return
    ///
    /// Db Pool
    async fn connect(cpath: &str) -> Result<Pool<MySql>, Error> {
        return MySqlPool::connect(cpath).await;
    }

    ///  Creates the weather_gov Db tables.
    ///
    /// # Arguments
    ///
    ///*'self'-the Db instance
    ///
    /// # Return
    ///
    /// Db Pool
    pub async fn create_tables(&mut self) -> Result<(), sqlx::Error>  {

        // Fatal if we cannot create tables
        let query_str_st =  format!("CREATE TABLE IF NOT EXISTS {} (call_id VARCHAR(5)
            PRIMARY KEY, name VARCHAR(80), latitude_deg FLOAT, longitude_deg FLOAT,
            elevation_m FLOAT, url VARCHAR(80))", self.station_table);
        let query_st = sqlx::query(query_str_st.as_str())
            .execute(&self.db_pool).await.expect("Fatal: could not create station metadata table");
        info!("Query result create station table: {:?}", query_st);

        let query_str_obs = format!("CREATE TABLE IF NOT EXISTS {} (station_id
        VARCHAR(20), timestamp_UTC VARCHAR(40), temperature_C FLOAT, temperature_F FLOAT,
        dewpoint_C FLOAT, dewpoint_F FLOAT, description VARCHAR(40), wind_dir FLOAT,
        wind_spd_km_h FLOAT, wind_spd_mi_h FLOAT, wind_gust_km_h FLOAT,
        wind_gust_mi_h FLOAT, baro_pres_pa FLOAT, baro_pres_inHg FLOAT,
        rel_humidity FLOAT, PRIMARY KEY (station_id, timestamp_UTC))",
        self.observation_table);
        let query_st_obs = sqlx::query(query_str_obs.as_str())
           .execute(&self.db_pool).await.expect("Fatal: could not create station \
                                                observation table");
        info!("Query result create observation table: {:?}", query_st_obs);

        Ok(())
    }

    ///  Adds a station to the weather_gov
    ///      Db station table.
    ///
    /// # Arguments
    ///
    ///*'self'-the Db instance
    ///*'rec'-a StationRecord
    ///
    /// # Return
    ///
    /// Result
    pub async fn put_station_record(&mut self, rec: StationRecord)
                                                 -> Result<String, sqlx::Error> {

        // Fatal if we cannot put station records

        let query_str = format!("REPLACE INTO {} (call_id, name, latitude_deg,
            longitude_deg, elevation_m, url) VALUES (?, ?, ?, ?, ?, ?)",
            self.station_table);
        let result = sqlx::query(query_str.as_str())
        .bind(rec.call_id)
        .bind(rec.name)
        .bind(rec.latitude_deg)
        .bind(rec.longitude_deg)
        .bind(rec.elevation_m)
        .bind(rec.url)
        .execute(&self.db_pool)
        .await
        .unwrap();

        let rstring = format!("Success: result: {:?}", result);
        Ok(rstring)

    }

    ///  Adds a station observation record to
    ///     the weather_gov db.
    ///
    /// # Arguments
    ///
    ///*'self'-the Db instance
    ///*'rec'-a ObservationRecord
    ///
    /// # Return
    ///
    /// Result
    pub async fn put_observation_record(&mut self, rec: ObservationRecord)
                                                 -> Result<String, sqlx::Error> {

        // Not fatal if we can't put an observation record

        let query_str = format!("INSERT INTO {} (station_id,
             timestamp_UTC, temperature_C, temperature_F, dewpoint_C,
             dewpoint_F, description, wind_dir, wind_spd_km_h, wind_spd_mi_h,
             wind_gust_km_h, wind_gust_mi_h, baro_pres_pa, baro_pres_inHg,
             rel_humidity) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
             self.observation_table);
         let result = sqlx::query(query_str.as_str())
        .bind(rec.station_id)
        .bind(rec.timestamp_UTC)
        .bind(rec.temperature_C)
        .bind(rec.temperature_F)
        .bind(rec.dewpoint_C)
        .bind(rec.dewpoint_F)
        .bind(rec.description)
        .bind(rec.wind_dir)
        .bind(rec.wind_spd_km_h)
        .bind(rec.wind_spd_mi_h)
        .bind(rec.wind_gust_km_h)
        .bind(rec.wind_gust_mi_h)
        .bind(rec.baro_pres_pa)
        .bind(rec.baro_pres_inHg)
        .bind(rec.rel_humidity)
        .execute(&self.db_pool)
        .await;
        //.unwrap();

        // Don't unwrap the result above, it will cause a crash on error.
        // The most common error is Duplicate record, which is not fatal.
        // All errors should be passed back to the caller and let caller
        // decide what to do.
        let rstring = format!("Success: result: {:?}", result);
        Ok(rstring)

    }

} // impl Db


