#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate mongodb;

use mongodb::{Client, options::ClientOptions};
use rocket::Response;
use rocket::http::Method;
use rocket_cors::{Guard, AllowedOrigins, AllowedHeaders, Responder};

#[get("/")]
fn index() -> &'static str {
    "Default route"
}

#[get("/orders")]
fn orders() -> &'static str {
	"Orders route"
}

#[get("/staff")]
fn staff() -> &'static str {
	"Staff route"
}

fn connect() {
    println!("Attempting connection");
    let mut client_options : ClientOptions =
    ClientOptions::parse("mongodb://localhost:27017").unwrap();
    client_options.app_name = Some("My App".to_string());
    let client : Client = Client::with_options(client_options).unwrap();

    for db_name in client.list_database_names(None).unwrap(){
        println!("{}", db_name);
    }    
}

fn main() {
    //connect();
    
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
    .mount("/api", routes![orders])
    .mount("/api", routes![staff])
    .attach(cors)
    .launch();
}
