use reqwest;
//use futures::executor::block_on;

pub struct Station {
    pub station_identifier:  String,
    pub station_url:         String,
    pub json_data:           String,
    pub observation_url:     String,

}


impl Station {

    pub fn new(id: String, stations_url: String) -> Station {
        let sid = id.clone();
        let surl = stations_url.clone();
        Self {
            station_identifier: id,
            //station_url: stations_url.to_string() + "/" + id,
            station_url: format!("{}{}", stations_url, sid),
            json_data: "".to_string(),
            observation_url: format!("{}{}/observations/latest", surl, sid),
        }
    }


    //pub async fn get_station_json(url: String) -> Result<String, Box<dyn std::error::Error>> {
    pub async fn get_station_json(&mut self) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let resp = client.get(&self.station_url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "Mozilla/5.0 (X11; Linux i686; rv:124.0) Gecko/20100101 Firefox/124.0")
            .send().await?;

        let rtext = resp.text().await?;
        self.json_data = rtext.clone();
        //println!("body = {rtext:?}");
        Ok(rtext)

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
