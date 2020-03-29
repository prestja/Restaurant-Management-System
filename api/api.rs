#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use rocket_contrib::{databases::mongodb};
use mongodb::doc;

mod ingredients;
mod staff;
mod orders;
mod tables;
mod schedules;
mod customers;

#[database("mongodb_logs")]
pub struct LogsDbConn(mongodb::db::Database);

fn main() 
{
    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions // attaches a CORS fairing to prevent security error on modern browsers
    {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();
    rocket::ignite()
	.mount("/api", routes![tables::get_all])
	.mount("/api", routes![tables::get])
	.mount("/api", routes![tables::post])
	.mount("/api", routes![tables::update_table])
	.mount("/api", routes![orders::get_all])
	.mount("/api", routes![orders::get])
	.mount("/api", routes![orders::post])
	.mount("/api", routes![ingredients::get_all])
	.mount("/api", routes![ingredients::get])
	.mount("/api", routes![ingredients::post])
	.mount("/api", routes![staff::get_all])
	.mount("/api", routes![staff::get])
	.mount("/api", routes![staff::post])
	.mount("/api", routes![customers::get_all])
	.mount("/api", routes![customers::get])
	.mount("/api", routes![customers::post])
	.mount("/api", routes![schedules::get_all])
	.mount("/api", routes![schedules::get])
	.mount("/api", routes![schedules::post])
	.attach(LogsDbConn::fairing())
	.attach(cors)
	.launch();
}
