use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

#[derive(Serialize, Deserialize)]
pub struct Payment {
	method: u32, // 0: card, 1: cash
	amount: f32
}

// Accepts JSON-formatted Notification and inserts into database
#[post("/?<table>", data = "<payment>")]
pub fn post(conn: LogsDbConn, table: u32, payment: Json<Payment>) -> String {
	println!("{}", table);
	let inner = payment.into_inner();
	let doc = doc! {
		"table": table,
		"status": {
			"$lt": 5 // where the status of the order is < 5 (not yet paid)
		}
	};
	let update = doc! { // push a new value onto the order
		"$push": {
			"payments": {
				"method": inner.method,
				"amount": inner.amount
			} 
		}
	};

	let coll = conn.collection("orders");
	if let Ok (result) = coll.find_one_and_update(doc, update, None) {
		if let Some(item) = result {
			let response = json!({
				"code": 200,
				"message": "Successfully updated payment information."
			});
			return serde_json::to_string(&response).unwrap();
		}
		else {
			let response = json!({
				"code": 404,
				"message": "Error updating payment information."
			});
			return serde_json::to_string(&response).unwrap();
		}		
	}			
	else {
		let response = json!({
			"code": 404,
			"message": "Error: either the table has no open tab or malformed information was provided."
		});
		return serde_json::to_string(&response).unwrap();
	}
}
