use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use mongodb::{doc, bson};
use serde_json;

#[get("/")]
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

#[get("/?<item>")]
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

/*
#[post("/")]
pub fn post(_conn: LogsDbConn) -> &'static str 
{
	let _coll = _conn.collection("ingredients");
	_coll.insert_one(doc!{ "item": "beef_patties" }, None).unwrap();
	return "Inserted an element into database";
}
*/

#[put("/?<name>&<count>")]
pub fn put_count(_conn: LogsDbConn, name: String, count: u32) -> String {
	let coll = _conn.collection("ingredients");
	let filter = doc! {"item": name};
	let _comp = doc! {
		"$set": {
			"count": count
		}
	};
	if let Ok (result) = coll.find_one_and_update(filter.clone(),_comp.clone(), None) {
		println!("Got a result");
		if let Some(item) = result {
			let response = json!({
				"code": 200,
				"message": "Successfully updated status for ingredient."
			});
			return serde_json::to_string(&response).unwrap();
		}
		let response = json!({
			"code": 404,
			"message": "Could not find ingredient to update."
		});
		return serde_json::to_string(&response).unwrap();
	}
	else {
		let response = json!({
			"code": 404,
			"message": "Error accessing database."
		});
		return serde_json::to_string(&response).unwrap();
	}
}
