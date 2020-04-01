use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use mongodb::{doc, bson};
use serde_json;

#[get("/schedules")]
pub fn get_all(_conn: LogsDbConn) -> String 
{
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("schedules");
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

#[get("/schedules/<schedule>")]
pub fn get(_conn: LogsDbConn, schedule: &rocket::http::RawStr) -> String 
{
	let mut str = String::from("[\n\t");
	let doc = doc!{"schedule": schedule.as_str()};
	let _coll = _conn.collection("schedules");
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

#[post("/schedules")]
pub fn post(_conn: LogsDbConn) -> &'static str 
{
	let _coll = _conn.collection("schedules");
	_coll.insert_one(doc!{ "schedule": "900-1700" }, None).unwrap();
	return "Inserted an element into database";
}
