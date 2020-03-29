use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;
use crate::serde_derive;

use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

#[derive(Serialize, Deserialize)]
pub struct Item {
	id: u32
}

#[derive(Serialize, Deserialize)]
pub struct Order {
	#[serde(default)] id: u32,
	#[serde(default)] status: u32,
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
pub fn post(_conn: LogsDbConn, order: Json<Order>) -> &'static str {
	let inner = order.into_inner(); // converts fron Json<Order> to just Order
	let j = serde_json::to_string_pretty(&inner); // stringifies

	println!("{}", j.unwrap());
	let doc = doc! {
		"id": inner.id,
		"status": inner.status
	};
	
	let _coll = _conn.collection("orders");
	_coll.insert_one(doc, None).unwrap();
	return "Post route";
}
