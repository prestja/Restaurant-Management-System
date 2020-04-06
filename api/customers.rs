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

#[derive(Serialize, Deserialize)]
pub struct Customer {
	#[serde(default)] phone: String,
	#[serde(default)] bday: String,
	#[serde(default)] email: String
}

#[get("/")]
pub fn get_all(_conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("customers");
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

#[get("/?<id>")]
pub fn get(_conn: LogsDbConn, id: u32) -> String {
	let mut str = String::from("[\n\t");
	let doc = doc!{"id": id};
	let _coll = _conn.collection("customers");
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
	if str.len() <= 3{
	return String::from("No entries found");
	}
	str.pop();
	str.pop();
	str.pop();
	str.push_str("\n]");
	return str;
}

#[post("/")]
pub fn post(_conn: LogsDbConn) -> &'static str 
{
	let _coll = _conn.collection("customers");
	_coll.insert_one(doc!{ "id": 32 }, None).unwrap();
	return "Inserted an element into database";
}

#[post("/", data = "<customer>")]
pub fn update_rewards(_conn: LogsDbConn, customer: Json<Customer>)
{
	let inner = customer.into_inner();
	let filter = doc!{
		"phone": inner.phone,
		"email": inner.email,
		"bday": inner.bday
	};
	let update = doc!{"$inc": {"quantity": 1}};
	let mut _upsert = mongodb::coll::options::FindOneAndUpdateOptions::new();
	_upsert.upsert = Some(true);
	let coll = _conn.collection("customers");
	let cursor = coll.find_one_and_update(filter.clone(), update.clone(), Some(_upsert.clone()));
}
