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

/*
pub fn get_all(conn: LogsDbConn) {

}
*/
