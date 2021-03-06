use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket::response::content;
use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

#[derive(Serialize, Deserialize)]
pub struct Customer {
	#[serde(default)] phone: String,
	#[serde(default)] bday: String,
	#[serde(default)] email: String
}

#[get("/")]
pub fn get_all(_conn: LogsDbConn) -> String {
	let mut str = String::from("[\n\t");
	let _coll = _conn.collection("customers");
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

#[get("/?<phone>")]
pub fn get(_conn: LogsDbConn, phone: String) -> String {
	let mut str = String::from("[\n\t");
	let doc = doc!{"phone": phone};
	let _coll = _conn.collection("customers");
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
	if str.len() <= 3{
	return String::from("No entries found");
	}
	str.pop();
	str.pop();
	str.pop();
	str.push_str("\n]");
	return str;
}

#[post("/", data = "<customer>")]
pub fn post(_conn: LogsDbConn, customer: Json<Customer>) -> String
{
	let inner = customer.into_inner();
	let doc = doc!{
		"phone": inner.phone,
		"email": inner.email,
		"bday": inner.bday,
		"visits": 1
	};
	let _coll = _conn.collection("customers");
	let result = _coll.insert_one(doc, None);
        if let Ok(inserted) = result {
                let response = json!({ // generate a response for the user
                        "code": 200,
                        "message": "Successfully inserted customer rewards into system."
                });
                return serde_json::to_string(&response).unwrap();
        }
        else {
                let response = json!({ // generate a response for the user
                        "code": 404,
                        "message": "Error inserting customer rewards into system."
                });
                return serde_json::to_string(&response).unwrap();
        }
}

#[post("/?<phone>")]
pub fn update_rewards(_conn: LogsDbConn, phone: String) -> String
{
	let filter = doc!{"phone": phone};
	let update = doc!{"$inc": {"visits": 1}};
	let mut _upsert = mongodb::coll::options::FindOneAndUpdateOptions::new();
        _upsert.upsert = Some(true);
        let coll = _conn.collection("customers");
        if let Ok(result) = coll.find_one_and_update(filter.clone(), update.clone(), Some(_upsert.clone())){
                if let Some(item) = result {
                        let response = json!({
                                "code": 200,
                                "message": "Successfully updated quantity for customer."
                        });
                        return serde_json::to_string(&response).unwrap();
                }
                let response = json!({
                        "code": 200,
                        "message": "Inserted new customer."
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
