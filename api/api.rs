#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
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
mod coupons;
mod timeclock;

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
	.mount("/api/tables", routes![tables::get_all])
	.mount("/api/tables", routes![tables::get])
	.mount("/api/tables", routes![tables::post])
	// order functions	
	.mount("/api/orders", routes![orders::get])
	.mount("/api/orders", routes![orders::get_id])
	.mount("/api/orders", routes![orders::get_status])
	.mount("/api/orders", routes![orders::get_comps])
	.mount("/api/orders", routes![orders::post])
	.mount("/api/orders", routes![orders::comp])
	.mount("/api/orders", routes![orders::get_table_orders])
	.mount("/api/orders", routes![orders::apply_promotion])
	// ingredients functions
	.mount("/api/ingredients", routes![ingredients::get_all])
	.mount("/api/ingredients", routes![ingredients::get])
	.mount("/api/ingredients", routes![ingredients::post_count])
	// staff functions
	.mount("/api/staff", routes![staff::get_all])
	.mount("/api/staff", routes![staff::get])
	.mount("/api/staff", routes![staff::get_names])
	.mount("/api/staff", routes![staff::get_login])
	.mount("/api/staff", routes![staff::post])
	.mount("/api/staff", routes![staff::delete])
	.mount("/api/staff", routes![staff::modify])
	// item functions
	.mount("/api/items/", routes![items::get_all])
	.mount("/api/items/", routes![items::get_category])
	.mount("/api/items/", routes![items::post_status])
	// notification functions
	.mount("/api/notifications", routes![notifications::get_all])
	.mount("/api/notifications", routes![notifications::post])
	// customer functions
	.mount("/api/customers", routes![customers::update_rewards])
	// coupons functions
	.mount("/api/coupons", routes![coupons::get_all])
	.mount("/api/coupons", routes![coupons::get_code])
	.mount("/api/coupons", routes![coupons::post])
	.mount("/api/timeclock", routes![timeclock::clock_in])
	.mount("/api/timeclock", routes![timeclock::clock_out])
	.mount("/api/timeclock", routes![timeclock::get_all])
	// mount and launch	
	.attach(LogsDbConn::fairing())
	.attach(cors)
	.launch();
}
