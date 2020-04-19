use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

// represtantion of a promotion
#[derive(Serialize, Deserialize)]
pub struct Promotion {
	name: String,
	start: String,
	end: String,
	amount: f32,
	category: i32
}

//Get all documents within the promotions database
#[get("/")]
pub fn get_all(_conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("promotions");
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

//Deserialize and post the new promotion to the database
#[post("/", data = "<promo>")]
pub fn post (conn: LogsDbConn, promo: Json<Promotion>) -> String {
	let inner = promo.into_inner();
	//create the document to insert
	let doc = doc! {
		"name": inner.name,
		"start": inner.start,
		"end": inner.end,
		"amount": inner.amount,
		"category": inner.category
	};
	
	let coll = conn.collection("promotions");
	//insert the new document
	if let Ok (result) = coll.insert_one(doc, None) {
		// create and return successful response
		let response = json!({
			"code": 200,
			"message": "Inserted promotion into system."
		});
		return serde_json::to_string(&response).unwrap();
	}
	else { // return an error code in the event of an error
		let response = json!({
			"code": 404,
			"message": "Error inserting promotion into system."
		});
		return serde_json::to_string(&response).unwrap();
	}	
}