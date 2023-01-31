
use data_api::ns_api::stations::StationAPI;

#[tokio::main]
pub async fn main(){
        StationAPI::send_request(true, 4326, false , 2, 0).await.unwrap();
}