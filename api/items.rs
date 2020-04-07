use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use mongodb::{doc, bson};

#[get("/")]
pub fn get_all (_conn: LogsDbConn) -> String
{	
	let mut _str = String::from("[\n\t");
	let _doc = doc!{};
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
		let filter = doc! {"_id": oid};
		let _comp = doc! {
			"$set": {
				"status": status
			}
		};
		let update = doc! {"status": status};
		if let Ok (result) = coll.find_one_and_update(filter.clone(),_comp.clone(), None) {
			println!("Got a result");
			if let Some(item) = result {
				let response = json!({
					"code": 200,
					"message": "Successfully updated status for item"
				});
				return serde_json::to_string(&response).unwrap();
			}
				let response = json!({
				"code": 404,
				"message": "Could not find item to update."
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
	else {
		let response = json!({
			"code": 404,
			"message": "Invalid or malformed object id."
		});
		return serde_json::to_string(&response).unwrap();	
	}
}
