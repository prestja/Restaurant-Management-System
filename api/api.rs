#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};

#[macro_use] extern crate rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use rocket_contrib::{databases::mongodb};
use mongodb::doc;
use mongodb::bson;
use serde_json;


#[database("mongodb_logs")]
struct LogsDbConn(mongodb::db::Database);

#[get("/")]
fn index(_conn: LogsDbConn) -> &'static str {  
    return "You are at the default route, this does nothing";
}

#[get("/orders")]
fn orders_get_all(_conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("orders");
	let cursor = _coll.find(None, None).unwrap();
	for result in cursor { // iterator
		if let Ok(item) = result { // error check the iterator and unwrap value to `item`
			let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
			let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
			str.push_str(&_json);
		}
		str.push_str(",\n\t"); // push separator characters for each new entry   	
	}
	str.pop(); 	// remove last unnecessary separator characters
	str.pop();
	str.pop();
	str.push_str("\n]");
	return str;
}

#[get("/orders/<id>")]
fn orders_get(id: u32) -> String {
    return format!("Orders GET. Number is {}", id);
}

#[post("/orders")]
fn orders_post(_conn: LogsDbConn) -> &'static str {
    let _coll = _conn.collection("orders");
    _coll.insert_one(doc!{ "id": 32 }, None).unwrap();
    return "Inserted an element into database";
}

#[get("/staff/<id>")]
fn staff(id: u32) -> String {
    return format!("Staff GET. Employee ID is {}", id);
}

fn main() {
    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();
    rocket::ignite()
    .mount("/api", routes![index])
    .mount("/api", routes![orders_get_all])
    .mount("/api", routes![orders_get])
    .mount("/api", routes![orders_post])
    .attach(LogsDbConn::fairing())
    .attach(cors)
    .launch();
}
