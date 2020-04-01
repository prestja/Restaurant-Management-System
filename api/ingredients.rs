use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use mongodb::{doc, bson};
use serde_json;

#[get("/ingredients")]
pub fn get_all(_conn: LogsDbConn) -> String 
{
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("ingredients");
	let cursor = _coll.find(None, None).unwrap();
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

#[get("/ingredients/<item>")]
pub fn get(_conn: LogsDbConn, item: &rocket::http::RawStr) -> String 
{
	let mut str = String::from("[\n\t");
	let doc = doc!{"item": item.as_str()};
	let _coll = _conn.collection("ingredients");
	let cursor = _coll.find(Some(doc.clone()), None).unwrap();
	for result in cursor {
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

#[post("/ingredients")]
pub fn post(_conn: LogsDbConn) -> &'static str 
{
	let _coll = _conn.collection("ingredients");
	_coll.insert_one(doc!{ "item": "beef_patties" }, None).unwrap();
	return "Inserted an element into database";
}
