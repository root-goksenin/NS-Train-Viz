
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
struct Geometry{
    r#type : String,
    coordinates : Vec<f64>
}

#[derive(Debug, Deserialize, Serialize)]
struct Properties{
    StationsNaam : String,
    StationsCode : String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Feature{
    r#type : String,
    geometry : Geometry, 
    properties : Properties
}
/*
The APIResponse we get from the NS Database. 
There is an expected JSON shape, therefore we can
leverage the power of the rust library to parse json files.
SERDE
{"type":"FeatureCollection",
"features":[
{"type":"Feature","geometry":{"type":"Point","coordinates":[6.0039990409392079,52.8805527761048]},"properties":{"StationsNaam":"Wolvega","StationsCode":"WV0000"}},
{"type":"Feature","geometry":{"type":"Point","coordinates":[4.5317605573261046,52.37569784728808]},"properties":{"StationsNaam":"Zandvoort aan Zee","StationsCode":"ZVT000"}}
]}
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct APIResponse{
    r#type : String,
    features : Vec<Feature>
}

/*
Station API for querying the NS database.
This module is set to be private as client does not need to access it and this just handles the
business logic.

*/
pub struct StationAPI{
}


impl StationAPI{
    pub async fn send_request(return_geometry : bool, out_sr : usize,
                        return_count_only : bool, result_record_count: usize, 
                        result_offset : usize) -> Option<APIResponse>{
        
        let send_url : String = format!("https://gateway.apiportal.ns.nl/Stationsplattegrond/Stations?returnGeometry={}&outSR={}&returnCountOnly={}&resultRecordCount={}&resultOffset={}", return_geometry, out_sr, return_count_only, result_record_count, result_offset);
        let client = reqwest::Client::new();
        let resp = client
                .get(send_url)
                .header("Ocp-Apim-Subscription-key", "d934c4acbc7e46fab567366822f0bf62")
                .send()
                .await.unwrap()
                .json::<APIResponse>()
                .await.unwrap();
        Some(resp)
    }           
    
}


