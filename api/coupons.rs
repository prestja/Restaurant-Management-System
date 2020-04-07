use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

//JSON Serialized datat type to be received with coupon information
#[derive(Serialize, Deserialize)]
pub struct Coupon {
	name: String,
	start: String,
	end: String,
	amount: f32,
	code: String
}

//Get all documents within the coupons database
#[get("/")]
pub fn get_all(_conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("coupons");
	let cursor = _coll.find(None, None).unwrap();
	for result in cursor
	{
		//if result is valid, set item to serialize
		if let Ok(item) = result 
		{
			let _bson = mongodb::to_bson(&item).unwrap();
			let _json = serde_json::ser::to_string(&_bson).unwrap();
			//add the serialized object to the return string
			str.push_str(&_json);
		}
		str.push_str(",\n\t");
	}
	//If there are no documents in the collection, return appropriate response
	if str.len() <= 3
	{
		return String::from("No entries found");
	}
	//Pop off the last comma, newline, and tab
	str.pop();
	str.pop();
	str.pop();
	//End the output string and return
	str.push_str("\n]");
	return str;
}

//Deserialize and post the new coupon to the database
#[post("/", data = "<coupon>")]
pub fn post(_conn: LogsDbConn, coupon: Json<Coupon>) -> String {
	let inner = coupon.into_inner();

	//create the document to insert
	let doc = doc!
	{
		"name": inner.name,
		"start": inner.start,
		"end": inner.end,
		"amount": inner.amount,
		"code": inner.code
	};

	//open coupons collection
	let _coll = _conn.collection("coupons");
	//insert the new document
	_coll.insert_one(doc, None).unwrap();
	//create and return response
	let response = json!({
		"code": 200,
		"message": "Inserted coupon into collection coupons"
	});
	return serde_json::to_string(&response).unwrap();
}
