use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use mongodb::{doc, bson};
use serde_json;

#[get("/staff")]
pub fn get_all(_conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("staff");
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

#[get("/staff/<id>")]
pub fn get(_conn: LogsDbConn, id: u32) -> String {
	let mut str = String::from("[\n\t");
	let doc = doc!{"id": id};
	let _coll = _conn.collection("staff");
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

#[post("/staff")]
pub fn post(_conn: LogsDbConn) -> &'static str {
	let _coll = _conn.collection("staff");
	_coll.insert_one(doc!{ "empid": 32 }, None).unwrap();
	return "Inserted an element into database";
}

#[post("/staff?<empid>&<tableid>")]
pub fn update_staff_table(_conn: LogsDbConn, empid: u32, tableid: u32) -> &'static str
{
        let _coll = _conn.collection("staff");
        let filter = doc!{"empid" => empid};
        let update = doc!{"$set" => {"tableid" => tableid}};

        _coll.update_one(filter, update, None).unwrap();
        return "Updated element in database";
}

#[post("/staff/manager?<first>&<last>&<empid>&<position>&<wage>&<phone_num>")]
pub fn manager_add_staff(_conn: LogsDbConn, first: &rocket::http::RawStr, last: &rocket::http::RawStr, empid: u32, position: u32, wage: f32, phone_num: &rocket::http::RawStr) -> &'static str
{
	let _coll = _conn.collection("staff");
	let doc = doc!{
	"firstName" => first.as_str(),
	"lastName" => last.as_str(),
	"empid" => empid,
	"position" => position,
	"wage" => wage,
	"phone" => phone_num.as_str()};
        _coll.insert_one(doc, None).unwrap();
        return "Inserted an element into database";
} 

