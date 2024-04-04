use reqwest;
//use serde::{Deserialize, Serialize};
//use serde_json::{Value};
//use log::{error, warn, info, debug};
use log::{warn, info, debug};


pub struct Station {
    pub station_identifier:      String,
    pub station_name:            String,
    pub station_url:             String,
    pub json_station_data:       String,
    pub json_station_serde_val:  serde_json::Value,
    pub observation_url:         String,
    pub longitude:               f64,
    pub latitude:                f64,
    pub elevation_meters:        f64,
    pub elevation_feet:          f64,

}


impl Station {

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
            longitude: 0.0,
            latitude: 0.0,
            elevation_meters: 0.0,
            elevation_feet: 0.0,
        }
    }


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
        Ok(rtext)
    }

    pub fn set_station_data(&mut self) {
        self.parse_json_station_name();
        self.parse_json_longitude();
        self.parse_json_latitude();
        self.parse_json_elevation();
    }

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

        info!("Station name: {:?}", self.station_name);
        info!("Station identifier: {:?}", self.station_identifier);
    }

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
            info!("Station longitude: {:?}", self.longitude);
        }
    }

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
            info!("Station latitude: {:?}", self.latitude);
        }
    }

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
            info!("Station elevation_meters: {:?}", self.elevation_meters);
            info!("Station elevation_feet: {:?}", self.elevation_feet);
        }
    }


} // impl Station


// use rust reqwest to get the json
//https://docs.rs/reqwest/latest/reqwest/

// Then, See here for deserializing JSON to an object, this uses serde
//https://stackoverflow.com/questions/75771097/safely-and-efficiently-processing-a-json-web-service-response-in-rust
//
//For example, what if a field is usually a number, but is sometimes a non-numeric string, or is omitted entirely?
//#[derive(Deserialize)]
//#[serde(untagged)]
//enum NumberOrString {
//    Number(f64),
//    String(String),
//}

//#[derive(Deserialize)]
//struct ApiResponse {
//    pub some_value: Option<NumberOrString>,
//}
//
//
//Now you can handle any of the three cases:
//
//match response.some_value {
//    None => { /* Value was null or missing */ }
//    Some(NumberOrString::Number(n)) => { /* Value was numeric */ }
//    Some(NumberOrString::String(s)) => { /* Value was a string */ }
//}
//
