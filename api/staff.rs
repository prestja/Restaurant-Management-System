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

//===================== WIP - Needs testing =======================================//
//Function needed to return list of all employee names
/*
// Was just told this was finished via Erika

#[get("/<first_name>&<last_name>")]
pub fn get(_conn: LogsDbConn, first_name: String, last_name: String) -> String {
	//create message string
	let mut str = String::from("[\n\t");
	//doc acts as our interim to pull information from the database and format it to be output to front
	let doc = doc!{"first_name": first_name, "last_name": last_name};
	//connect to database
	let _coll = _conn.collection("staff");
	let cursor = _coll.find(Some(doc.clone()), None).unwrap();
	//For each result in database
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
*/
//===================== WIP - Needs testing =======================================//


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
		"message": "Inserted order into collection orders"
	});
	return serde_json::to_string(&response).unwrap();
}
