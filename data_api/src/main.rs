
#[macro_use] extern crate rocket;
use data_api::ns_api::end_points::get_stations;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/stations", routes![get_stations])
        .launch()
        .await?;
    Ok(())
}