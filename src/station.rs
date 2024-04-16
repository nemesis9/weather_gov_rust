use reqwest;
use log::{error, warn, debug};
use std::fmt;
use std::io;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

/// Represents a database station record.
pub struct StationRecord {
    pub call_id:         String,
    pub name:            String,
    pub latitude_deg:    f64,
    pub longitude_deg:   f64,
    pub elevation_m:     f64,
    pub url:             String,
}

/// Enables debugging a database station record.
impl fmt::Debug for StationRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StationRecord")
            .field("\n        call_id", &self.call_id)
            .field("\n        name", &self.name)
            .field("\n        latitude_deg", &self.latitude_deg)
            .field("\n        longitude_deg", &self.longitude_deg)
            .field("\n        elevation_m", &self.elevation_m)
            .field("\n        url", &self.url)
            .finish()
    }
}


/// Represents database station observation record.
#[allow(non_snake_case)]
pub struct ObservationRecord {
    pub station_id:       String,
    pub timestamp_UTC:    String,
    pub temperature_C:    f64,
    pub temperature_F:    f64,
    pub dewpoint_C:       f64,
    pub dewpoint_F:       f64,
    pub description:      String,
    pub wind_dir:         f64,
    pub wind_spd_km_h:    f64,
    pub wind_spd_mi_h:    f64,
    pub wind_gust_km_h:   f64,
    pub wind_gust_mi_h:   f64,
    pub baro_pres_pa:     f64,
    pub baro_pres_inHg:   f64,
    pub rel_humidity:     f64,
}

/// Enables debugging a database observation record.
impl fmt::Debug for ObservationRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ObservationRecord")
            .field("\n        station_id", &self.station_id)
            .field("\n        timestamp_UTC", &self.timestamp_UTC)
            .field("\n        temperature_C", &self.temperature_C)
            .field("\n        temperature_F", &self.temperature_F)
            .field("\n        description", &self.description)
            .field("\n        wind_dir", &self.wind_dir)
            .field("\n        wind_spd_km_h", &self.wind_spd_km_h)
            .field("\n        wind_spd_mi_h", &self.wind_spd_mi_h)
            .field("\n        wind_gust_km_h", &self.wind_gust_km_h)
            .field("\n        wind_gust_mi_h", &self.wind_gust_mi_h)
            .field("\n        baro_pres_pa", &self.baro_pres_pa)
            .field("\n        baro_pres_inHg", &self.baro_pres_inHg)
            .field("\n        rel_humidity", &self.rel_humidity)
            .finish()
    }
}

/// Implmentation of a weather_gov station.
pub struct Station {
    pub station_identifier:          String,
    pub station_name:                String,
    pub station_url:                 String,
    pub json_station_data:           String,
    pub json_station_serde_val:      serde_json::Value,
    pub observation_url:             String,
    pub latest_observation_data:     String,
    pub json_observation_serde_val:  serde_json::Value,
    pub longitude:                   f64,
    pub latitude:                    f64,
    pub elevation_meters:            f64,
    pub elevation_feet:              f64,

}


/// Implmentation of methods for struct Station.
impl Station {

    ///  Creates a new station instance.
    ///
    /// # Arguments
    ///
    ///*'id'-the station id
    ///*'stations_url' - the main statons_url
    ///
    /// # Return
    ///
    /// Station instance
    pub fn new(id: String, stations_url: String) -> Station {
        let sid = id.clone();
        let surl = stations_url.clone();
        Self {
            station_identifier: id,
            station_name: "".to_string(),
            station_url: format!("{}{}", stations_url, sid),
            json_station_data: "".to_string(),
            json_station_serde_val: serde_json::Value::Null ,
            observation_url: format!("{}{}/observations/latest", surl, sid),
            latest_observation_data: "".to_string(),
            json_observation_serde_val:  serde_json::Value::Null,
            longitude: 0.0,
            latitude: 0.0,
            elevation_meters: 0.0,
            elevation_feet: 0.0,
        }
    }


    ///  Gets the station meta data.
    ///
    /// # Arguments
    ///
    ///*'self'-the object instance
    ///
    /// # Return
    ///
    /// Result
    pub async fn get_station_json(&mut self) -> Result<String, reqwest::Error> {
        // api.weather.gov requires User-Agent be set, but reqwest does not
        // set one. See weather.gov.
        let client = reqwest::Client::new();
        let resp = client.get(&self.station_url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "Mozilla/5.0 (X11; Linux i686; rv:124.0)\
                                            Gecko/20100101 Firefox/124.0")
            .send().await?;

        let rtext = resp.text().await?;
        self.json_station_data = rtext.clone();
        // Need to match here, not use ?, because the error type is not reqwest::Error
        // If we cannot get the station meta json, we won't be able to create a db record,
        //   so we panic. Another reason we need to match.
        self.json_station_serde_val = match serde_json::from_str(&self.json_station_data) {
            Ok(v) => v,
            Err(e) => panic!("Could not parse json station data: {:?}", e),
        };

        self.set_station_data();

        Ok(rtext)
    }


    ///  Auxiliary function to set data
    ///         on the station instance.
    ///
    /// # Arguments
    ///
    ///*'self'-the station instance
    ///
    /// # Return
    ///
    /// None
    pub fn set_station_data(&mut self) {
        self.parse_json_station_name();
        self.parse_json_longitude();
        self.parse_json_latitude();
        self.parse_json_elevation();
    }

    ///  Get station name from station json.
    ///
    /// # Arguments
    ///
    ///*'self'-the station instance
    ///
    /// # Return
    ///
    /// None
    fn parse_json_station_name(&mut self) {

        debug!("parse json station name called");

        //Returns an Option, the value if successful, None otherwise
        let n = self.json_station_serde_val["properties"]["name"].as_str();

        let nm: String = match n {
            Some(f) => String::from(f),
            None => self.station_identifier.clone(),
        };

        if nm == self.station_identifier {
            warn!("Could not get name from json, \
                  setting name to station_identifier: {:?}", self.station_identifier);
        } else {
            self.station_name = nm;
        }
    }

    ///  Get station longitude from station json.
    ///
    /// # Arguments
    ///
    ///*'self'-the station instance
    ///
    /// # Return
    ///
    /// None
    fn parse_json_longitude(&mut self) {

        debug!("parse json longitude called");

        //Returns an Option, the value if successful, None otherwise
        let l = self.json_station_serde_val["geometry"]["coordinates"][0].as_f64();

        let long = match l {
            Some(f) => f,
            None  => 0.0
        };

        self.longitude = long;
        if self.longitude == 0.0 {
            warn!("WARNING: Parsing error: station {:?} longitude \
                  set to zero.", self.station_identifier);
        } else {
            debug!("Station longitude: {:?}", self.longitude);
        }
    }

    ///  Get station latitude from station json.
    ///
    /// # Arguments
    ///
    ///*'self'-the station instance
    ///
    /// # Return
    ///
    /// None
    fn parse_json_latitude(&mut self) {

        debug!("parse json latitude called");

        //Returns an Option, the value if successful, None otherwise
        let l = self.json_station_serde_val["geometry"]["coordinates"][1].as_f64();

        let lat = match l {
            Some(f) => f,
            None  => 0.0
        };

        self.latitude = lat;
        if self.latitude == 0.0 {
            warn!("WARNING: Parsing error: station {:?} latitude \
                  set to zero.", self.station_identifier);
        } else {
            debug!("Station latitude: {:?}", self.latitude);
        }
    }

    ///  Get station elevation from station json.
    ///
    /// # Arguments
    ///
    ///*'self'-the station instance
    ///
    /// # Return
    ///
    /// None
    fn parse_json_elevation(&mut self) {

        debug!("parse json elevation called");

        //Returns an Option, the value if successful, None otherwise
        let e = self.json_station_serde_val["properties"]["elevation"]["value"].as_f64();

        let ele = match e {
            Some(f) => f,
            None  => 0.0
        };

        self.elevation_meters = ele;
        self.elevation_feet = self.elevation_meters * 3.28084;
        if self.elevation_meters == 0.0 {
            warn!("WARNING: Parsing error: station {:?} latitude \
                  set to zero.", self.station_identifier);
        } else {
            debug!("Station elevation_meters: {:?}", self.elevation_meters);
            debug!("Station elevation_feet: {:?}", self.elevation_feet);
        }
    }

    ///  Get station record suitable for
    ///      db station record.
    ///
    /// # Arguments
    ///
    ///*'self'-the station instance
    ///
    /// # Return
    ///
    /// StationRecord
    pub fn get_station_record(&self) -> StationRecord {
        StationRecord {
            call_id:         self.station_identifier.clone(),
            name:            self.station_name.clone(),
            latitude_deg:    self.latitude,
            longitude_deg:   self.longitude,
            elevation_m:     self.elevation_meters,
            url:             self.station_url.clone(),
        }
    }

    ///  Get station observation record suitable for
    ///      db station observation record
    ///
    /// # Arguments
    ///
    ///*'self'-the station instance
    ///
    /// # Return
    ///
    /// ObservationRecord or Error
    ///    generic error is used as there are multiple error types and dont want to panic.
    ///    This should just return to caller.
    ///    Failing to get an observation is not fatal.
    ///    GenericResult accomplishes what is wanted, but may lose info about what
    ///    exacty failed. However, looks like I can wrap the actual error in the Generic Error
    ///    using format!.
    ///    On the bright side, this call rarely, if ever, fails.
    pub async fn get_latest_observation_data(&mut self)  -> GenericResult<ObservationRecord> {
        // api.weather.gov requires User-Agent be set, but reqwest does not
        // set one. See weather.gov.
        let client = reqwest::Client::new();
        let resp = match client.get(&self.observation_url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "Mozilla/5.0 (X11; Linux i686; rv:124.0)\
                                            Gecko/20100101 Firefox/124.0")
            .send().await {
             Ok(resp) => resp,
             Err(e) =>  {
                    error!("Error getting latest observation: {:?}", e);
                    let io_error = io::Error::new(io::ErrorKind::Other,
                                   format!("get observation response failed: {:?}", e));
                    return Err(GenericError::from(io_error));
                    },

            };

        let rtext = match resp.text().await {

             Ok(rtext) => rtext,
             Err(e) =>  {
                    error!("Error getting observation response: {:?}", e);
                    let io_error = io::Error::new(io::ErrorKind::Other,
                                   format!("get observation response text failed: {:?}", e));
                    return Err(GenericError::from(io_error));
                    },

        };
        self.latest_observation_data = rtext.clone();

        // serde_json::from_str does not emit a reqwest::Error on failure.
        // By default, cannot use ? operator, but using a GenericResult allows this.
        //self.json_observation_serde_val
        //    serde_json::from_str(&self.latest_observation_data)?;
        let json_obs_serde = match  serde_json::from_str(&self.latest_observation_data) {
            Ok(obs) => obs,

            Err(e) =>  {
                    error!("Error getting observation response: {:?}", e);
                    let io_error = io::Error::new(io::ErrorKind::Other,
                                   format!("get observation json failed: {:?}", e));
                    return Err(GenericError::from(io_error));
                    },
        };

        self.json_observation_serde_val = json_obs_serde;
        let obs = self.preprocess_observation();
        Ok(obs)
    }

    ///   Helper for get_latest_observation_data,
    ///      adds items that are not natively in the json
    ///      and handles null values.
    ///
    /// # Arguments
    ///
    ///*'self'-the station instance
    ///
    /// # Return
    ///
    /// ObservationRecord
    fn preprocess_observation(&mut self) -> ObservationRecord {
        let mut obs = ObservationRecord {
            station_id:       self.station_identifier.clone(),
            timestamp_UTC:    "".to_string(),
            temperature_C:    0.0,
            temperature_F:    0.0,
            dewpoint_C:       0.0,
            dewpoint_F:       0.0,
            description:      "".to_string().clone(),
            wind_dir:         0.0,
            wind_spd_km_h:    0.0,
            wind_spd_mi_h:    0.0,
            wind_gust_km_h:   0.0,
            wind_gust_mi_h:   0.0,
            baro_pres_pa:     0.0,
            baro_pres_inHg:   0.0,
            rel_humidity:     0.0,
        };
        let res =  self.json_observation_serde_val["properties"]["timestamp"].as_str();
        match res {
            Some(v) => { obs.timestamp_UTC = v.to_string(); },
            None => { obs.timestamp_UTC = "".to_string(); },
        }

        let res =  self.json_observation_serde_val["properties"]["temperature"]["value"].as_f64();
        match res {
            Some(v) => { obs.temperature_C = v; obs.temperature_F = v * (9.0/5.0) + 32.0; },
            None => { obs.temperature_C = -999.99; obs.temperature_F = -999.99; },
        }

        let res =  self.json_observation_serde_val["properties"]["dewpoint"]["value"].as_f64();
        match res {
            Some(v) => { obs.dewpoint_C = v; obs.dewpoint_F = v * (9.0/5.0) + 32.0; }
            None => { obs.dewpoint_C = -999.99; obs.dewpoint_F = -999.99; },
        }


        let res =  self.json_observation_serde_val["properties"]["textDescription"].as_str();
        match res {
            Some(v) => { obs.description = v.to_string(); },
            None => { obs.description = "".to_string(); },
        }


        let res =  self.json_observation_serde_val["properties"]["windDirection"]["value"].as_f64();
        match res {
            Some(v) => { obs.wind_dir = v; },
            None => { obs.wind_dir = -999.99; },
        }

        let res =  self.json_observation_serde_val["properties"]["windSpeed"]["value"].as_f64();
        match res {
            Some(v) => { obs.wind_spd_km_h = v; obs.wind_spd_mi_h = v * 0.6213712 },
            None => { obs.wind_spd_km_h = -999.99; obs.wind_spd_mi_h = -999.99},
        }

        let res =  self.json_observation_serde_val["properties"]["windGust"]["value"].as_f64();
        match res {
            Some(v) => { obs.wind_gust_km_h = v; obs.wind_gust_mi_h = v * 0.6213712 },
            None => { obs.wind_gust_km_h = -999.99; obs.wind_gust_mi_h = -999.99},
        }

        let res =  self.json_observation_serde_val["properties"]["barometricPressure"]["value"].as_f64();
        match res {
            Some(v) => { obs.baro_pres_pa = v; obs.baro_pres_inHg = v * 0.00029529983071445; },
            None => { obs.baro_pres_pa = -999.99; obs.baro_pres_inHg = -999.99},
        }

        let res =  self.json_observation_serde_val["properties"]["relativeHumidity"]["value"].as_f64();
        match res {
            Some(v) => { obs.rel_humidity = v; },
            None => { obs.rel_humidity = -999.99 ; },
        }


        obs
    }

} // impl Station



