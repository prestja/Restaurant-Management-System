#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};

#[macro_use] extern crate rocket_contrib;
use rocket_contrib::databases::mongodb;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;

#[database("mongodb_logs")]
struct LogsDbConn(mongodb::db::Database);

#[get("/")]
fn index(conn: LogsDbConn) -> &'static str {
   	let collection = conn.collection("orders");
   	println!("{}", collection.name());
    return "Default GET";
}


#[get("/orders")]
fn orders(conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let collection = conn.collection("orders");
	let mut cursor = collection.find(None, None);
	for entries in cursor{
		str.push_str(entries);
		str.push_str(",\n");
	}
	str.push_str("]");
	return str;
}

#[get("/orders/<ordernum>")]
fn orders_get(ordernum: u8) -> String {
        return format!("Orders GET. Number is {}", ordernum);
}

#[post("/orders/<ordernum>")]
fn orders_post(ordernum: u8) -> String {
        return format!("Orders POST. Number is {}", ordernum);
}

#[get("/staff/<emp_id>")]
fn staff(emp_id: u8) -> String {
        return format!("Staff GET. Employee ID is {}", emp_id);
}


/*
fn get_client() -> mongodb::Client {
    let mut client_options : ClientOptions =
    ClientOptions::parse("mongodb://localhost:27017").unwrap();
    client_options.app_name = Some("My App".to_string());
    let client : Client = Client::with_options(client_options).unwrap();

	return client;
}

*/
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
    .mount("/api", routes![orders_get])
    .mount("/api", routes![orders_post])
    .attach(LogsDbConn::fairing())
    .attach(cors)
    .launch();
}
