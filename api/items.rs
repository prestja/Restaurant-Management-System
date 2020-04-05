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
	//println!("{}", id);
	//let result = bson::oid::ObjectId::with_string(id.as_str());
	let coll = _conn.collection("items");
	let result = coll.find_one(Some(doc! { "_id": bson::oid::ObjectId::with_string("5e83fdcc90c5a642e00f6241").unwrap() }), None);
    if let Ok(opt) = result {
    	if let Some(item) = opt {
    		println!("Found!");
    	}
    	else {
    		println!("Not found");
    	}
    }
    else {
    	println!("Result is not okay");
    } 
    return String::from("something");
}
