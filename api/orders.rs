use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;
use crate::serde_derive;

use rocket::response::content;
use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};
use mongodb::coll::options;

#[derive(Serialize, Deserialize)]
pub struct Item {
	id: u32
}

#[derive(Serialize, Deserialize)]
pub struct Order {
	table: u32,
	
	// default values, not supplied by the front-end
	#[serde(default)] id: u32,
	#[serde(default)] status: u32
}

#[get("/")]
pub fn get(_conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("orders");
	let cursor = _coll.find(None, None).unwrap();
	for result in cursor {
		if let Ok(item) = result {
			let _bson = mongodb::to_bson(&item).unwrap();
			let _json = serde_json::ser::to_string(&_bson).unwrap();
			str.push_str(&_json);
		}
		str.push_str(",\n\t");
	}
	if str.len() <= 3{
		return String::from("No entries found");
	}
	str.pop();
	str.pop();
	str.pop();
	str.push_str("\n]");
	return str;
}

#[get("/?<status>", rank = 6)]
pub fn get_status(_conn: LogsDbConn, status: u32) -> String {
	let mut str = String::from("[\n\t");
	let doc = doc!{"status": status};
	let _coll = _conn.collection("orders");
	let cursor = _coll.find(Some(doc.clone()), None).unwrap();
	for result in cursor 
	{
		if let Ok(item) = result 
		{
			let _bson = mongodb::to_bson(&item).unwrap();
			let _json = serde_json::ser::to_string(&_bson).unwrap();
			str.push_str(&_json);
		}
		str.push_str(",\n\t");
	}
	if str.len() <= 3
	{
		return String::from("No entries found");
	}
	str.pop();
	str.pop();
	str.pop();
	str.push_str("\n]");
	return str;
}

#[get("/?<id>", rank = 5)]
pub fn get_id(_conn: LogsDbConn, id: u32) -> String 
{
	let mut str = String::from("[\n\t");
	let doc = doc!{"id": id};
	let _coll = _conn.collection("orders");
	let cursor = _coll.find(Some(doc.clone()), None).unwrap();
	for result in cursor 
	{
		if let Ok(item) = result 
		{
			let _bson = mongodb::to_bson(&item).unwrap();
			let _json = serde_json::ser::to_string(&_bson).unwrap();
			str.push_str(&_json);
		}
		str.push_str(",\n\t");
	}
	if str.len() <= 3
	{
		return String::from("No entries found");
	}
	str.pop();
	str.pop();
	str.pop();
	str.push_str("\n]");
	return str;
}

#[post("/", data = "<order>")]
pub fn post(_conn: LogsDbConn, order: Json<Order>) -> String {
	let inner = order.into_inner(); // converts fron Json<Order> to just Order

	let doc = doc! // create a new document based upon deserialized object
	{
		"table": inner.table,
		"id": inner.id,
		"status": inner.status
	};
	
	let _coll = _conn.collection("orders");
	let options = mongodb::coll::options::IndexOptions {
		background: None,
		expire_after_seconds: None,
		name: None,
		sparse: None,
		storage_engine: None,
		unique: Some(true),
		version: None,
		default_language: None,
		language_override: None,
		text_version: None,
		weights: None,
		sphere_version: None,
		bits: None,
		max: None,
		min: None,
		bucket_size: None
	};
	//_coll.create_index(doc!{"id": 1}, None);


	_coll.insert_one(doc, None).unwrap();
	let response = json!({ // generate a response for the user
		"code": 200,
		"message": "Inserted order into collection orders"
	});

	return serde_json::to_string(&response).unwrap();
}
