use reqwest;
//use serde::{Deserialize, Serialize};
use serde_json::{Value};
//use serde_yaml::from_str;
use log::{error, warn, info, debug};


//#[derive(Serialize, Deserialize)]
//struct Geo {
//    id: String
//}

pub struct Station {
    pub station_identifier:      String,
    pub station_url:             String,
    pub json_station_data:       String,
    //pub json_station_serde_val:  serde_json::Value,
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
            station_url: format!("{}{}", stations_url, sid),
            json_station_data: "".to_string(),
            //json_station_serde_val: None,
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
        Ok(rtext)
    }

    pub fn parse_json_longitude(&mut self, json: &str) -> Result<(), serde_json::error::Error> {

        debug!("parse json longitude called");

        let v: Value = serde_json::from_str(json)?;

        debug!("Station long: {:?}", v["geometry"]["coordinates"][0]);
        let l = v["geometry"]["coordinates"][0].as_f64();

        let long = match l {
          Some(f) => f,
          None  => 0.0
        };

        self.longitude = long;
        if self.longitude == 0.0 {
            warn!("WARNING: Parsing error: station {:?} longitude \
                  set to zero.", self.station_identifier);
        } else {
            info!("self longitude: {:?}", self.longitude);
        }
        Ok(())
    }

    pub fn parse_json_latitude(&mut self, json: &str) -> Result<(), serde_json::error::Error> {

        debug!("parse json latitude called");

        let v: Value = serde_json::from_str(json)?;

        debug!("Station lat: {:?}", v["geometry"]["coordinates"][1]);
        let l = v["geometry"]["coordinates"][1].as_f64();

        let long = match l {
          Some(f) => f,
          None  => 0.0
        };

        self.latitude = long;
        if self.latitude == 0.0 {
            warn!("WARNING: Parsing error: station {:?} latitude \
                  set to zero.", self.station_identifier);
        } else {
            info!("self latitude: {:?}", self.latitude);
        }
        Ok(())
    }


}


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
