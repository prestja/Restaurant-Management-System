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
        if str.len() <= 3{
          return String::from("No entries found");
        }
	str.pop(); 	// remove last unnecessary separator characters
	str.pop();
	str.pop();
	str.push_str("\n]");
	return str;
}

#[get("/orders/<id>")]
fn orders_get(_conn: LogsDbConn, id: u32) -> String {
    	let mut str = String::from("[\n\t");
    	let doc = doc!{"id": id}; //create a document to search for within the database
    	let _coll = _conn.collection("orders");
    	let cursor = _coll.find(Some(doc.clone()), None).unwrap(); //search for the defined document
    	for result in cursor { // iterator
        	if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                     	let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                     	let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                     	str.push_str(&_json);
            	}
            	str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[post("/orders")]
fn orders_post(_conn: LogsDbConn) -> &'static str {
    let _coll = _conn.collection("orders");
    _coll.insert_one(doc!{ "id": 32 }, None).unwrap();
    return "Inserted an element into database";
}

#[get("/staff")]
fn staff_get_all(_conn: LogsDbConn) -> String {
        let mut str = String::from("[\n\t");
        let _coll = _conn.collection("staff");
        let cursor = _coll.find(None, None).unwrap();
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[get("/staff/<id>")]
fn staff_get(_conn: LogsDbConn, id: u32) -> String {
	let mut str = String::from("[\n\t");
        let doc = doc!{"empid": id}; //create a document to search for within the database
        let _coll = _conn.collection("staff");
        let cursor = _coll.find(Some(doc.clone()), None).unwrap(); //search for the defined document
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
	if str.len() <= 3{
	  return String::from("No entries found");
	}
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[post("/staff")]
fn staff_post(_conn: LogsDbConn) -> &'static str {
    let _coll = _conn.collection("staff");
    _coll.insert_one(doc!{ "empid": 32 }, None).unwrap();
    return "Inserted an element into database";
}

#[get("/customers")]
fn customers_get_all(_conn: LogsDbConn) -> String {
        let mut str = String::from("[\n\t");
        let _coll = _conn.collection("customers");
        let cursor = _coll.find(None, None).unwrap();
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[get("/customers/<id>")]
fn customers_get(_conn: LogsDbConn, id: u32) -> String {
        let mut str = String::from("[\n\t");
        let doc = doc!{"id": id}; //create a document to search for within the database
        let _coll = _conn.collection("customers");
        let cursor = _coll.find(Some(doc.clone()), None).unwrap(); //search for the defined document
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[post("/customers")]
fn customers_post(_conn: LogsDbConn) -> &'static str {
    let _coll = _conn.collection("customers");
    _coll.insert_one(doc!{ "id": 32 }, None).unwrap();
    return "Inserted an element into database";
}

#[get("/ingredients")]
fn ingredients_get_all(_conn: LogsDbConn) -> String {
        let mut str = String::from("[\n\t");
        let _coll = _conn.collection("ingredients");
        let cursor = _coll.find(None, None).unwrap();
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[get("/ingredients/<item>")]
fn ingredients_get(_conn: LogsDbConn, item: &rocket::http::RawStr) -> String {
        let mut str = String::from("[\n\t");
        let doc = doc!{"item": item.as_str()}; //create a document to search for within the database
        let _coll = _conn.collection("ingredients");
        let cursor = _coll.find(Some(doc.clone()), None).unwrap(); //search for the defined document
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[post("/ingredients")]
fn ingredients_post(_conn: LogsDbConn) -> &'static str {
    let _coll = _conn.collection("ingredients");
    _coll.insert_one(doc!{ "item": "beef_patties" }, None).unwrap();
    return "Inserted an element into database";
}

#[get("/schedules")]
fn schedules_get_all(_conn: LogsDbConn) -> String {
        let mut str = String::from("[\n\t");
        let _coll = _conn.collection("schedules");
        let cursor = _coll.find(None, None).unwrap();
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[get("/schedules/<schedule>")]
fn schedules_get(_conn: LogsDbConn, schedule: &rocket::http::RawStr) -> String {
        let mut str = String::from("[\n\t");
        let doc = doc!{"schedule": schedule.as_str()}; //create a document to search for within the database
        let _coll = _conn.collection("schedules");
        let cursor = _coll.find(Some(doc.clone()), None).unwrap(); //search for the defined document
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[post("/schedules")]
fn schedules_post(_conn: LogsDbConn) -> &'static str {
    let _coll = _conn.collection("schedules");
    _coll.insert_one(doc!{ "schedule": "900-1700" }, None).unwrap();
    return "Inserted an element into database";
}

#[get("/tables")]
fn tables_get_all(_conn: LogsDbConn) -> String {
        let mut str = String::from("[\n\t");
        let _coll = _conn.collection("tables");
        let cursor = _coll.find(None, None).unwrap();
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[get("/tables/<id>")]
fn tables_get(_conn: LogsDbConn, id: u32) -> String {
        let mut str = String::from("[\n\t");
        let doc = doc!{"id": id}; //create a document to search for within the database
        let _coll = _conn.collection("tables");
        let cursor = _coll.find(Some(doc.clone()), None).unwrap(); //search for the defined document
        for result in cursor { // iterator
                if let Ok(item) = result { // error check the iterator and unwrap value to `item`
                        let _bson = mongodb::to_bson(&item).unwrap(); // convert item to bson
                        let _json = serde_json::ser::to_string(&_bson).unwrap(); // convert item to json string
                        str.push_str(&_json);
                }
                str.push_str(",\n\t"); // push separator characters for each new entry
        }
        if str.len() <= 3{
          return String::from("No entries found");
        }
        str.pop();      // remove last unnecessary separator characters
        str.pop();
        str.pop();
        str.push_str("\n]");
        return str;
}

#[post("/tables")]
fn tables_post(_conn: LogsDbConn) -> &'static str {
    let _coll = _conn.collection("tables");
    _coll.insert_one(doc!{ "id": 32 }, None).unwrap();
    return "Inserted an element into database";
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
    .mount("/api", routes![staff_get_all])
    .mount("/api", routes![staff_get])
    .mount("/api", routes![staff_post])
    .mount("/api", routes![customers_get_all])
    .mount("/api", routes![customers_get])
    .mount("/api", routes![customers_post])
    .mount("/api", routes![ingredients_get_all])
    .mount("/api", routes![ingredients_get])
    .mount("/api", routes![ingredients_post])
    .mount("/api", routes![schedules_get_all])
    .mount("/api", routes![schedules_get])
    .mount("/api", routes![schedules_post])
    .mount("/api", routes![tables_get_all])
    .mount("/api", routes![tables_get])
    .mount("/api", routes![tables_post])
    .attach(LogsDbConn::fairing())
    .attach(cors)
    .launch();
}
