#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate mongodb;

use mongodb::{Client, options::ClientOptions};

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
    connect();
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/", routes![orders])
    .mount("/", routes![staff])
    .launch();
}
