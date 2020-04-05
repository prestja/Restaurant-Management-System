#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;

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
mod items;
mod notifications;

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
	/*
	.mount("/api", routes![tables::get_all])
	.mount("/api", routes![tables::get])
	.mount("/api", routes![tables::post])	
	*/
	//.mount("/api/", routes![orders::get])
	.mount("/api/orders", routes![orders::get])
	.mount("/api/orders", routes![orders::get_id])
	.mount("/api/orders", routes![orders::get_status])	
	.mount("/api/orders", routes![orders::post])
	.mount("/api/orders", routes![orders::comp])
	/*
	.mount("/api", routes![tables::post])
	.mount("/api", routes![tables::update_table])
	*/
	.mount("/api/ingredients", routes![ingredients::get_all])
	.mount("/api/ingredients", routes![ingredients::get])
	.mount("/api/ingredients", routes![ingredients::post])
	.mount("/api/ingredients", routes![ingredients::post_status])
	.mount("/api/staff", routes![staff::get_all])
	.mount("/api/staff", routes![staff::get])
	.mount("/api/staff", routes![staff::get_login])
	.mount("/api/staff", routes![staff::post])
	//.mount("/api", routes![staff::update_staff_table])
	//.mount("/api", routes![staff::manager_add_staff])
	
	.mount("/api/items/", routes![items::get_category])
	.mount("/api/items/", routes![items::post_status])
	.mount("/api/notifications", routes![notifications::get_all])
	.mount("/api/notifications", routes![notifications::post])
	/*
	.mount("/api", routes![customers::get_all])
	.mount("/api", routes![customers::get])
	.mount("/api", routes![customers::post])
	.mount("/api", routes![schedules::get_all])
	.mount("/api", routes![schedules::get])
	.mount("/api", routes![schedules::post])
	*/	
	.attach(LogsDbConn::fairing())
	.attach(cors)
	.launch();
}
