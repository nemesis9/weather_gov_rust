
use std::collections::HashMap;
//use mysql::*;
//use mysql::prelude::*;
use sqlx::{Pool, MySql, Error, MySqlPool};
use async_std::task;


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

    pub async fn connect(cpath: &str) -> Result<Pool<MySql>, Error> {
        return MySqlPool::connect(cpath).await;
    }

    pub async fn create_tables(&mut self) -> Result<Pool<MySql>, sqlx::Error>  {

        let pool_str = format!("mysql://{}:{}@{}:3306/{}", self.user, self.password,
                                self.host, self.database);

        println!("pool_str: {:?}", pool_str);

        let result = task::block_on(Db::connect(pool_str.as_str()));

        match result {
            Err(err) => {
                println!("Cannot connect to database [{}]", err.to_string());
                return Err(err);
            }        

            Ok(pool) => {
                println!("Connected to database successfully.");
                //let query = sqlx::query("CREATE TABLE IF NOT EXISTS 
                //station_rust (call_id VARCHAR(5) PRIMARY KEY, name VARCHAR(80), latitude_deg 
                //FLOAT, longitude_deg FLOAT, elevation_m FLOAT, url VARCHAR(80))").execute(&pool).await.unwrap();
                let query_str_st =  format!("CREATE TABLE IF NOT EXISTS {} (call_id VARCHAR(5) 
                    PRIMARY KEY, name VARCHAR(80), latitude_deg FLOAT, longitude_deg FLOAT, 
                    elevation_m FLOAT, url VARCHAR(80))", self.station_table);
                let query_st = sqlx::query(query_str_st.as_str()).execute(&pool).await.unwrap();
                println!("Query result create station table: {:?}", query_st);

                let query_str_obs = format!("CREATE TABLE IF NOT EXISTS {} (station_id 
                VARCHAR(20), timestamp_UTC VARCHAR(40), temperature_C FLOAT, temperature_F FLOAT,
                dewpoint_C FLOAT, dewpoint_F FLOAT, description VARCHAR(40), wind_dir FLOAT,
                wind_spd_km_h FLOAT, wind_spd_mi_h FLOAT, wind_gust_km_h FLOAT,
                wind_gust_mi_h FLOAT, baro_pres_pa FLOAT, baro_pres_inHg FLOAT,
                rel_humidity FLOAT, PRIMARY KEY (station_id, timestamp_UTC))", 
                self.observation_table);
                let query_st_obs = sqlx::query(query_str_obs.as_str())
                    .execute(&pool).await.unwrap();
                println!("Query result create observation table: {:?}", query_st_obs);

                Ok(pool)
            }
        }
    }

        //let opts = Opts::try_from(pool_str.as_str());
        //let url = get_opts();
        //let conn = Conn::new(opts);
        //    //mysql::Pool::new("mysql://weather_user:weather_pass@localhost:3306/weather_gov")
        //    mysql::Pool::new(mysql::Opts::From(&pool_str))
        //    .unwrap();

        //let mut conn = pool.get_conn().expect("Failed to get a connection from the pool");
        //let mut conn = pool.get_conn()?;

        //let query_str = format!("CREATE TABLE IF NOT EXISTS {} (call_id VARCHAR(5) PRIMARY KEY,
        //                     name VARCHAR(80), latitude_deg FLOAT, longitude_deg FLOAT,
        //                     elevation_m FLOAT, url VARCHAR(80))", self.station_table);

        //conn.query_drop(&query_str).expect("Failed to create station table");
        //conn.query_drop(
        //r"CREATE TABLE IF NOT EXISTS station_rust (call_id VARCHAR(5) PRIMARY KEY,
        //name VARCHAR(80), latitude_deg FLOAT, longitude_deg FLOAT, elevation_m FLOAT,
        //url VARCHAR(80))"
        //).expect("Failed to create station table");

        //println!("Station Table created successfully!");
        //let query_str = format!("CREATE TABLE IF NOT EXISTS {} (station_id VARCHAR(20),
        //        timestamp_UTC VARCHAR(40), temperature_C FLOAT, temperature_F FLOAT,
        //        dewpoint_C FLOAT, dewpoint_F FLOAT, description VARCHAR(40), wind_dir FLOAT,
        //        wind_spd_km_h FLOAT, wind_spd_mi_h FLOAT, wind_gust_km_h FLOAT,
        //        wind_gust_mi_h FLOAT, baro_pres_pa FLOAT, baro_pres_inHg FLOAT,
        //        rel_humidity FLOAT, PRIMARY KEY (station_id, timestamp_UTC))", 
        //        self.observation_table);

        //conn.query_drop(&query_str).expect("Failed to create observation table");

        //conn.query_drop(
        //r"CREATE TABLE IF NOT EXISTS observation_rust (station_id VARCHAR(20),
        //timestamp_UTC VARCHAR(40), temperature_C FLOAT, temperature_F FLOAT,
        //dewpoint_C FLOAT, dewpoint_F FLOAT, description VARCHAR(40), wind_dir FLOAT,
        //wind_spd_km_h FLOAT, wind_spd_mi_h FLOAT, wind_gust_km_h FLOAT,
        //wind_gust_mi_h FLOAT, baro_pres_pa FLOAT, baro_pres_inHg FLOAT,
        //rel_humidity FLOAT, PRIMARY KEY (station_id, timestamp_UTC))"
        //).expect("Failed to create observation table");

        //println!("Observation Table created successfully!");

} // impl Db
