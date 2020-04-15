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
pub struct Employee {
	#[serde(default)] first_name: String,
	#[serde(default)] last_name: String,
	#[serde(default)] id : String,
	#[serde(default)] password: String,
	#[serde(default)] wage : f32,
	#[serde(default)] phone: String,
	#[serde(default)] position: u32,
}

#[get("/")]
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

#[get("/<id>")]
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

#[get("/names")]
pub fn get_names(_conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let mut _filter = mongodb::coll::options::FindOptions::new();
	_filter.projection = Some(doc!{"names" : 1, "_id" : 0});
	let _coll = _conn.collection("staff");
	let cursor = _coll.find(None, Some(_filter.clone())).unwrap();
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

#[get("/login?<id>&<password>")]
pub fn get_login(_conn: LogsDbConn, id: String, password: String) -> String {	
	let doc = doc!{"id": id, "password": password};
	let _coll = _conn.collection("staff");
	let mut cursor = _coll.find(Some(doc.clone()), None).unwrap();
	if let Some(result) = cursor.next() 
	{
		if let Ok(item) = result 
		{
			let _bson = mongodb::to_bson(&item).unwrap();
			let response = json!({
				"code": 200,
				"message": "Succesfully logged in.",
				"data": _bson
			});
			return serde_json::to_string(&response).unwrap();
		}
	} 

	let response = json!({
		"code": 403,
		"message": "Invalid employee ID or password!"
	});
	return serde_json::to_string(&response).unwrap();
}

#[post("/", data = "<employee>")]
pub fn post(_conn: LogsDbConn, employee: Json<Employee>) -> String {
	let inner = employee.into_inner();
	let doc = doc!
	{
		"first_name": inner.first_name,
		"last_name": inner.last_name,
		"id": inner.id,
		"password": inner.password,
		"wage" : inner.wage,
		"phone": inner.phone,
		"position": inner.position
	};
	
	let _coll = _conn.collection("staff");
	_coll.insert_one(doc, None).unwrap();
	let response = json!({
		"code": 200,
		"message": "Successfully inserted employee."
	});
	return serde_json::to_string(&response).unwrap();
}

#[post("/modify?<id>", data = "<employee>")]
pub fn modify(_conn: LogsDbConn, id: String, employee: Json<Employee>) -> String {
	let inner = employee.into_inner();
	let filter = doc! {
		"id": id
	};
	let replacement = doc! {
		"first_name": inner.first_name,
		"last_name": inner.last_name,
		"id": inner.id,
		"password": inner.password,
		"wage": inner.wage,
		"phone": inner.phone,
		"position": inner.position
	};
	let coll = _conn.collection("staff");
	if let Ok(result) = coll.find_one_and_replace(filter, replacement, None) {
		if let Some(item) = result {
			let response = json!({
				"code": 200,
				"message": "Successfully updated employee."
			});
			return serde_json::to_string(&response).unwrap();
		}
		else {
			let response = json!({
				"code": 404,
				"message": "Error during insertion process: invalid or malformed employee."
			});
			return serde_json::to_string(&response).unwrap();
		}			
	}
	else {
		let response = json!({
				"code": 404,
				"message": "Could not find the requested employee."
		});
		return serde_json::to_string(&response).unwrap();
	}
}

#[post("/delete?<id>")]
pub fn delete(_conn: LogsDbConn, id: String) -> String {
	let filter = doc! {
		"id": id
	};
	let coll = _conn.collection("staff");
	if let Ok (result) = coll.delete_one(filter, None) {
		let response = json!({
			"code": 200,
			"message": "Removed the requested employee."
		});
		return serde_json::to_string(&response).unwrap();
	}
	let response = json!({
		"code": 404,
		"message": "Could not find that employee"
	});
	return serde_json::to_string(&response).unwrap();
}
