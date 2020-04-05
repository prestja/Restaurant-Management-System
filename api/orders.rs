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
pub struct Item {
	id: mongodb::oid::ObjectId
}

#[derive(Serialize, Deserialize)]
pub struct Order {
	// required values, should be supplied by the front-end
	table: u32,
	items: serde_json::Value,

	// default values, not required for deserialization
	#[serde(default)] id: u32,
	#[serde(default)] status: u32  // ordered = 0, NeedStaff = 1, NeedManager = 2, Ready = 3, Served = 4, Closed = 5	
}

#[get("/", rank = 3)]
pub fn get(_conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("orders");
	let cursor = _coll.find(None, None).unwrap();
	for result in cursor {
		if let Ok(item) = result {
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

#[get("/?<status>", rank = 0)]
pub fn get_status(_conn: LogsDbConn, status: u32) -> String {
	let mut str = String::from("[\n\t");
	let doc = doc!{"status": status};
	let _coll = _conn.collection("orders");
	let cursor = _coll.find(Some(doc.clone()), None).unwrap();
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

#[get("/?<id>", rank = 1)]
pub fn get_id (_conn: LogsDbConn, id: String) -> String {	
	let mut _str = String::from("[\n\t");
	let _doc = doc!{"_id": id};
	let _coll = _conn.collection("orders");
	let _cursor = _coll.find(Some(_doc.clone()), None).unwrap();
	for result in _cursor 
	{
		println!("Result");
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

#[get("/?<tableid>", rank = 2)]
pub fn get_table_orders(_conn: LogsDbConn, tableid: u32) -> String {
	let mut doc_list = String::from("");
        let _doc = doc!{"table": tableid};
	let mut _filter = mongodb::coll::options::FindOptions::new();
	_filter.projection = serde::export::Some(doc!{"items": 1, "_id": 0});
        let _coll = _conn.collection("orders");
	let _cursor = _coll.find(Some(_doc.clone()), Some(_filter.clone())).unwrap();
	for result in _cursor
	{
		if let Ok(item) = result
		{
			if item != doc!{}
			{
				let _bson = mongodb::to_bson(&item).unwrap();
				let _json = serde_json::ser::to_string(&_bson).unwrap();
				doc_list.push_str(&_json);
				doc_list.push_str("\n");
			}
		}
	}
	if doc_list.len() <= 1
	{
		return String::from("No entries found");
	}
	doc_list.pop();
	return doc_list;
}


#[post("/", data = "<order>")]
pub fn post(_conn: LogsDbConn, order: Json<Order>) -> String {
	let inner = order.into_inner(); // converts fron Json<Order> to just Order
	let doc = doc! // create a new document based upon deserialized object
	{
		"table": inner.table,
		"id": inner.id,
		"items": inner.items,
		"status": inner.status,
		"total": 43.19,
		"tip": 5.00 
	};
	
	let _coll = _conn.collection("orders");
	_coll.insert_one(doc, None).unwrap();
	let response = json!({ // generate a response for the user
		"code": 200,
		"message": "Inserted order into collection orders"
	});
	return serde_json::to_string(&response).unwrap();
}

#[post("/comp?<table>&<amount>")]
pub fn comp (_conn: LogsDbConn, table: u32, amount: f32) -> String {
	// this document is for the query (what to find)
	let _doc = doc! {
		"table": table,
		"status": {
			"$lt": 5 // where the status of the order is < 5 (not yet paid)
		},
		"total": {
			"$gte": amount
		}
	};
	// this document applies new or updates values
	let _comp = doc! {
		"$set": {
			"comp": amount
		}
	};

	let _coll = _conn.collection("orders");
	let _result = _coll.find_one_and_update(_doc.clone(), _comp.clone(), None).unwrap();
	
	// if there was a result updated
	if let Some(item) = _result {
		let response = json!({
			"code": 200,
			"message": "Successfully compensated order."
		});
		return serde_json::to_string(&response).unwrap();
	}
	// otherwise, output an error
	let response = json!({
		"code": 404,
		"message": "No unpaid orders could be found for the specified table."
	});
	return serde_json::to_string(&response).unwrap();
}
