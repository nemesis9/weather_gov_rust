
use std::collections::HashMap;
use sqlx::{Pool, MySql, Error, MySqlPool};
use async_std::task;
use crate::station::StationRecord;
use crate::station::ObservationRecord;

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

        //println!("pool_str: {:?}", pool_str);

        let result = task::block_on(Db::connect(pool_str.as_str()));

        match result {
            Err(err) => {
                println!("Cannot connect to database [{}]", err.to_string());
                return Err(err);
            }

            Ok(pool) => {
                println!("Connected to database successfully.");

                let query_str_st =  format!("CREATE TABLE IF NOT EXISTS {} (call_id VARCHAR(5)
                    PRIMARY KEY, name VARCHAR(80), latitude_deg FLOAT, longitude_deg FLOAT,
                    elevation_m FLOAT, url VARCHAR(80))", self.station_table);
                let query_st = sqlx::query(query_str_st.as_str())
                    .execute(&pool).await.expect("Fatal: could not create station metadata table");
                println!("Query result create station table: {:?}", query_st);

                let query_str_obs = format!("CREATE TABLE IF NOT EXISTS {} (station_id
                VARCHAR(20), timestamp_UTC VARCHAR(40), temperature_C FLOAT, temperature_F FLOAT,
                dewpoint_C FLOAT, dewpoint_F FLOAT, description VARCHAR(40), wind_dir FLOAT,
                wind_spd_km_h FLOAT, wind_spd_mi_h FLOAT, wind_gust_km_h FLOAT,
                wind_gust_mi_h FLOAT, baro_pres_pa FLOAT, baro_pres_inHg FLOAT,
                rel_humidity FLOAT, PRIMARY KEY (station_id, timestamp_UTC))",
                self.observation_table);
                let query_st_obs = sqlx::query(query_str_obs.as_str())
                   .execute(&pool).await.expect("Fatal: could not create station observation table");
                println!("Query result create observation table: {:?}", query_st_obs);

                Ok(pool)
            }
        }
    }

    pub async fn put_station_record(&mut self, rec: StationRecord) -> Result<String, sqlx::Error> {

        let pool_str = format!("mysql://{}:{}@{}:3306/{}", self.user, self.password,
                                self.host, self.database);

        //println!("pool_str: {:?}", pool_str);

        let result = task::block_on(Db::connect(pool_str.as_str()));

        match result {
            Err(err) => {
                println!("Cannot connect to database [{}]", err.to_string());
                return Err(err);
            },
            Ok(pool) => {
                let result = sqlx::query("REPLACE INTO station_rust (call_id, name, latitude_deg, longitude_deg, elevation_m, url) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(rec.call_id)
                .bind(rec.name)
                .bind(rec.latitude_deg)
                .bind(rec.longitude_deg)
                .bind(rec.elevation_m)
                .bind(rec.url)
                .execute(&pool)
                .await
                .unwrap();

                let rstring = format!("Sucess: result: {:?}", result);
                Ok(rstring)
            }
        }

    }


    pub async fn put_observation_record(&mut self, rec: ObservationRecord) -> Result<String, sqlx::Error> {

        let pool_str = format!("mysql://{}:{}@{}:3306/{}", self.user, self.password,
                                self.host, self.database);

        //debug!("put_obs_record: pool_str: {:?}", pool_str);

        let result = task::block_on(Db::connect(pool_str.as_str()));

        match result {
            Err(err) => {
                println!("Cannot connect to database [{}]", err.to_string());
                return Err(err);
            },
            Ok(pool) => {
                let result = sqlx::query("INSERT INTO observation_rust  (station_id,
                 timestamp_UTC, temperature_C, temperature_F, dewpoint_C,
                 dewpoint_F, description, wind_dir, wind_spd_km_h, wind_spd_mi_h,
                 wind_gust_km_h, wind_gust_mi_h, baro_pres_pa, baro_pres_inHg,
                 rel_humidity) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
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
                .execute(&pool)
                .await;
                //.unwrap();

                let rstring = format!("Sucess: result: {:?}", result);
                Ok(rstring)
            }
        }

    }

} // impl Db



//  https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/


// The CPP code to insert records
//bool
//Db::put_station_record(std::map<std::string, std::variant<std::string, float>>& station_record) {
//    try {
//
//        wlog(logDEBUG) << "put station record: establishing connection";
//        sql::Driver* driver = sql::mariadb::get_driver_instance();
//        std::string connect_info = "jdbc:mariadb://" + m_host + ":3306/" + m_database;
//        sql::SQLString url(connect_info);
//        sql::Properties properties({{"user", "weather_user"}, {"password", "weather_pass"}});
//        sql::Connection *conn = driver->connect(url, properties);
//
//        sql::PreparedStatement  *prep_stmt;
//        prep_stmt = conn->prepareStatement("REPLACE INTO station_cpp(call_id, name, latitude_deg, longitude_deg, elevation_m, url) VALUES (?, ?, ?, ?, ?, ?)");
//
//        prep_stmt->setString(1, std::get<std::string>(station_record["call_id"]));
//        prep_stmt->setString(2, std::get<std::string>(station_record["name"]));
//        prep_stmt->setFloat(3, std::get<float>(station_record["latitude_deg"]));
//        prep_stmt->setFloat(4, std::get<float>(station_record["longitude_deg"]));
//        prep_stmt->setFloat(5, std::get<float>(station_record["elevation_m"]));
//        prep_stmt->setString(6, std::get<std::string>(station_record["url"]));
//
//        prep_stmt->execute();
//        delete prep_stmt;
//        delete conn;
//
//    } catch (sql::SQLException& e) {
//        wlog(logERROR) << "Exception inserting station: " << e.what();
//    }
//
//    return true;
//}



//sql::PreparedStatement  *prep_stmt;
//        prep_stmt = conn->prepareStatement("INSERT INTO observation_cpp (station_id,"
//            "timestamp_UTC, temperature_C, temperature_F, dewpoint_C,"
//            "dewpoint_F, description, wind_dir, wind_spd_km_h, wind_spd_mi_h,"
//            "wind_gust_km_h, wind_gust_mi_h, baro_pres_pa, baro_pres_inHg,"
//            "rel_humidity) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
//
//        prep_stmt->setString(1, std::get<std::string>(obs["station_id"]));
//        prep_stmt->setString(2, std::get<std::string>(obs["timestamp_UTC"]));
//        prep_stmt->setFloat(3, std::get<float>(obs["temperature_C"]));
//        prep_stmt->setFloat(4, std::get<float>(obs["temperature_F"]));
//        prep_stmt->setFloat(5, std::get<float>(obs["dewpoint_C"]));
//        prep_stmt->setFloat(6, std::get<float>(obs["dewpoint_F"]));
//        prep_stmt->setString(7, std::get<std::string>(obs["description"]));
//        prep_stmt->setFloat(8, std::get<float>(obs["wind_dir"]));
//        prep_stmt->setFloat(9, std::get<float>(obs["wind_spd_km_h"]));
//        prep_stmt->setFloat(10, std::get<float>(obs["wind_spd_mi_h"]));
//        prep_stmt->setFloat(11, std::get<float>(obs["wind_gust_km_h"]));
//        prep_stmt->setFloat(12, std::get<float>(obs["wind_gust_mi_h"]));
//        prep_stmt->setFloat(13, std::get<float>(obs["baro_pres_pa"]));
//        prep_stmt->setFloat(14, std::get<float>(obs["baro_pres_inHg"]));
//        prep_stmt->setFloat(15, std::get<float>(obs["rel_humidity"]));
//
//        prep_stmt->execute();
//        delete prep_stmt;
//        delete conn;

