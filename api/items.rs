use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;
use crate::serde_derive;

use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

#[derive(Serialize, Deserialize)]
pub struct Item {
	name: String,
	category: u32,
	price: f32,
	nutrition: String,
	imgPath: String,
	#[serde(default)] ingredients: serde_json::Value,
	#[serde(default)] allergen: String,	
	#[serde(default)] vegan: bool,
	#[serde(default)] vegetarian: bool,
	#[serde(default)] status: u32 // status defaults to 1 upon insertion
}

#[get("/")]
pub fn get_all (_conn: LogsDbConn) -> String {	
	let mut _str = String::from("[\n\t");
	let _doc = doc!{};
	let _coll = _conn.collection("items");
	let _cursor = _coll.find(Some(_doc.clone()), None).unwrap();
	for result in _cursor {
		if let Ok(item) = result {
			let _bson = mongodb::to_bson(&item).unwrap();
			let _json = serde_json::ser::to_string(&_bson).unwrap();
			_str.push_str(&_json);
		}
		_str.push_str(",\n\t");
	}
	if _str.len() <= 3 {
		return String::from("No entries found");
	}
	_str.pop();
	_str.pop();
	_str.pop();
	_str.push_str("\n]");
	return _str;
}

#[get("/?<category>")]
pub fn get_category (_conn: LogsDbConn, category: u32) -> String {	
	let mut _str = String::from("[\n\t");
	let _doc = doc!{"category": category};
	let _coll = _conn.collection("items");
	let _cursor = _coll.find(Some(_doc.clone()), None).unwrap();
	for result in _cursor {
		if let Ok(item) = result {
			let _bson = mongodb::to_bson(&item).unwrap();
			let _json = serde_json::ser::to_string(&_bson).unwrap();
			_str.push_str(&_json);
		}
		_str.push_str(",\n\t");
	}
	if _str.len() <= 3 {
		return String::from("No entries found");
	}
	_str.pop();
	_str.pop();
	_str.pop();
	_str.push_str("\n]");
	return _str;
}

#[post("/", data = "<item>")]
pub fn post (_conn: LogsDbConn, item: Json<Item>) -> String {
	let inner = item.into_inner(); // deserializes the Json-formatted item
	let doc = doc! {
		"name": inner.name,
		"category": inner.category,
		"price": inner.price,
		"nutrition": inner.nutrition,
		"ingredients": inner.ingredients,
		"allergen": inner.allergen,
		"vegan": inner.vegan,
		"vegetarian": inner.vegetarian,
		"status": 1,
		"imgPath": inner.imgPath
	};
	
	let _coll = _conn.collection("items");
	let result = _coll.insert_one(doc, None);
	if let Ok(inserted) = result {
		let response = json!({ // generate a response for the user
			"code": 200,
			"message": "Successfully inserted item into system."
		});
		return serde_json::to_string(&response).unwrap();
	}
	else {
		let response = json!({ // generate a response for the user
			"code": 404,
			"message": "Error inserting item into system."
		});
		return serde_json::to_string(&response).unwrap();
	}
}

#[post("/modify?<id>&<price>")]
pub fn post_modify_price (conn: LogsDbConn, id: String, price: f32) -> String {
	let cast = bson::oid::ObjectId::with_string(id.as_str());
	let coll = conn.collection("items");
	if let Ok(oid) = cast {
		let filter = doc! {"_id": oid};		
		let update = doc! {
			"$set": {
				"price": price
			}
		};
		if let Ok (result) = coll.find_one_and_update(filter.clone(), update.clone(), None) {
			if let Some(item) = result {
				let response = json!({
					"code": 200,
					"message": "Successfully updated price for item"
				});
				return serde_json::to_string(&response).unwrap();
			}
			else {
				let response = json!({
					"code": 404,
					"message": "Could not find item to update."
				});
				return serde_json::to_string(&response).unwrap();
			}				
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
