
use crate::ns_api::stations::{StationAPI, APIResponse};
use rocket::get;
use rocket::serde::json::Json;

#[get("/get_geodata/<total>")]
pub async fn get_stations(total: usize) -> Json<APIResponse> {
    if let Some(response) =  StationAPI::send_request(true, 4326, false, total, 0).await{
        return Json(response);
    }
    todo!();

}