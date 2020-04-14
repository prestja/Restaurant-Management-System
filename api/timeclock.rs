use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

//JSON Serialized datat type to be received with coupon information
#[derive(Serialize, Deserialize)]
pub struct TimeInfo {
	id: String,
	password: String
}

#[get("/")]
pub fn get_all(_conn:LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("timeclock");
	let cursor = _coll.find(None, None).unwrap();
	for result in cursor {
		if let Ok(item) = result {
			let _bson = mongodb::to_bson(&item).unwrap();
			let _json = serde_json::ser::to_string(&_bson).unwrap();
			str.push_str(&_json);
		}
		str.push_str(",\n\t");
	}
	if str.len() <= 3 {
		return String::from("No entries found");
	}
	str.pop();
	str.pop();
	str.pop();
	str.push_str("\n]");
	return str;
}
#[post("/clockin", data = "<timeinfo>")]
pub fn clock_in(conn: LogsDbConn, timeinfo: Json<TimeInfo>) -> String {
	let inner = timeinfo.into_inner();
	let coll = conn.collection("timeclock");	
	//create the document to insert
	let previous = doc! {
		"id": inner.id.clone(), // id must be cloned in order to prevent move errors
		"clocked_out": false
	};
	// check to see if this employee has previously clocked in without clocking out
	if let Ok (result) = coll.find_one(Some(previous), None) {
		if let Some(item) = result {
			let response = json!({
				"code": 404,
				"message": "Could not clock in. Please clock out first"
			});
			return serde_json::to_string(&response).unwrap();
		}
	}
	let insert = doc! {
		"id": inner.id,
		"password": inner.password,
		"clocked_out": false
	};

	let update = doc! {		
		"$currentDate": { // $currentDate only works with upsert (update)
			"clocked_in": true
		},		
	};

	let mut options = mongodb::coll::options::FindOneAndUpdateOptions::new();
	options.upsert = Some(true);
	// upsert the new document
	if let Ok(result) = coll.find_one_and_update(insert, update, Some(options)) {
		let response = json!({
			"code": 200,
			"message": "Successfully clocked in"
		});
		return serde_json::to_string(&response).unwrap();
	}
	else {
		let response = json!({
			"code": 404,
			"message": "Error clocking in!"
		});
		return serde_json::to_string(&response).unwrap();
	}
}

#[post("/clockout", data = "<timeinfo>")]
pub fn clock_out(conn: LogsDbConn, timeinfo: Json<TimeInfo>) -> String {
	let inner = timeinfo.into_inner();
	//create the document to insert
	let doc = doc! {
		"id": inner.id,
		"password": inner.password,
		"clocked_out": false
	};

	//open coupons collection
	let coll = conn.collection("timeclock");
	let update = doc!{"$currentDate": { "clocked_out": true}};
	//insert the new document
	if let Ok(result) = coll.find_one_and_update(doc.clone(), update.clone(), None) { 
		if let Some(item) = result {
			let response = json!({
				"code": 200,
				"message": "Successfully clocked out."
			});
			return serde_json::to_string(&response).unwrap();
		} else {
			let response = json!({
				"code": 404,
				"message": "Error clocking out! Make sure you have previously clocked in."
			});
			return serde_json::to_string(&response).unwrap();
		}
		
	}
	else {
		let response = json!({
			"code": 404,
			"message": "Unknown error while clocking out!."
		});
		return serde_json::to_string(&response).unwrap();
	}
}
