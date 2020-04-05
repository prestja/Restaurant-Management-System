use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;
use crate::serde_derive;

use rocket::response::content;
use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};
use mongodb::coll::options;
use mongodb::oid;
use bson::oid::ObjectId;

#[get("/?<category>")]
pub fn get_category (_conn: LogsDbConn, category: u32) -> String
{	
	let mut _str = String::from("[\n\t");
	let _doc = doc!{"category": category};
	let _coll = _conn.collection("items");
	let _cursor = _coll.find(Some(_doc.clone()), None).unwrap();
	for result in _cursor 
	{
		if let Ok(item) = result 
		{
			let _bson = mongodb::to_bson(&item).unwrap();
			let _json = serde_json::ser::to_string(&_bson).unwrap();
			_str.push_str(&_json);
		}
		_str.push_str(",\n\t");
	}
	if _str.len() <= 3
	{
		return String::from("No entries found");
	}
	_str.pop();
	_str.pop();
	_str.pop();
	_str.push_str("\n]");
	return _str;
}

#[post("/?<id>&<status>")]
pub fn post_status(_conn: LogsDbConn, id: String, status: u32) -> String {
	let cast = bson::oid::ObjectId::with_string(id.as_str());
	let coll = _conn.collection("items");
	if let Ok(oid) = cast {
		if let Ok (result) = coll.find_one(Some(doc! { "_id": oid }), None) {
			if let Some(item) = result {
				return String::from("Valid item");
			}
		}
		return String::from("Valid cast");
	}
	else {
		return String::from("Invalid cast");
	}
}
