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
	#[serde(default)] status: u32,  // ordered = 0, NeedStaff = 1, NeedManager = 2, Ready = 3, Served = 4, Closed = 5
	#[serde(default)] substitutions: String,
	#[serde(default)] allergies: String,
	#[serde(default)] total: f32
}

#[get("/", rank = 4)]
pub fn get_all(_conn: LogsDbConn) -> String {
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

#[post("/status?<table>&<status>")]
pub fn post_status(conn: LogsDbConn, table: u32, status: u32) -> String {
	let doc = doc! {
		"table": table,
		"status": {
			"$lt": 5 // where the status of the order is < 5 (not yet paid)
		}
	}; // find the most recent order for the table
	let update = doc! {
		"$set": {
			"status": status
		}
	}; // apply the user-specified status	
	let coll = conn.collection("orders");
	if let Ok (result) = coll.find_one_and_update(doc, update, None) {
		let response = json!({
			"code": 200,
			"message": "Successfully updated status for order."
		});		
		return serde_json::to_string(&response).unwrap();
	}
	let response = json!({
		"code": 404,
		"message": "Could not find an order to update."
	});		
	return serde_json::to_string(&response).unwrap();
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
	if _str.len() <= 3 {
		return String::from("No entries found");
	}
	_str.pop();
	_str.pop();
	_str.pop();
	_str.push_str("\n]");
	return _str;
}

// removes the specified item from the 
#[post("/remove?<table>&<id>")]
pub fn remove (conn: LogsDbConn, table: u32, id: String) -> String {
	let coll = conn.collection("orders");
	let cast = bson::oid::ObjectId::with_string(id.as_str());
	let filter = doc! {
		"table": table,
		"status": {
			"$lt": 5 // where the status of the order is < 5 (not yet paid)
		}
	};
	
	if let Ok (oid) = cast {
		let update = doc! { // pulls value from array and removes it
			"$pull": {
				"items": oid
			}
		};
		if let Ok(result) = coll.find_one_and_update(filter, update, None) {
			if let Some(item) = result {
				let response = json!({
            		"code": 200,
            		"message": "Successfully removed item from order."
        		});
       		 	return serde_json::to_string(&response).unwrap();
			}
			else {
				let response = json!({
            		"code": 404,
            		"message": "Failed to find item or order for the specified table."
        		});
        		return serde_json::to_string(&response).unwrap();
			}			
		}
		else {
			let response = json!({
        		"code": 404,
        		"message": "Database error."
    		});
    		return serde_json::to_string(&response).unwrap();
		}
	}
	else {
		let response = json!({
    		"code": 404,
    		"message": "Invalid or malformed ObjectID."
		});
		return serde_json::to_string(&response).unwrap();
	}
}

#[get("/?<tableid>", rank = 2)]
pub fn get_table_orders(_conn: LogsDbConn, tableid: u32) -> String {
	let mut doc_list = String::from("[\n\t");
    let _doc = doc!{"table": tableid};
    let _coll = _conn.collection("orders");
	let _itemcoll = _conn.collection("items");
	let _cursor = _coll.find(Some(_doc.clone()), None).unwrap(); //search for the specified table in the orders database
	for result in _cursor {
		if let Ok(item) = result {
			if let Ok(array) = item.get_array("items"){ //get the array of items that are stored
				for value in array{ //go through each item and extract its id
					let itemid = value.as_object_id().unwrap();
					let itemdoc = doc!{"_id": itemid.clone()};
					let itemcursor = _itemcoll.find(Some(itemdoc.clone()), None).unwrap(); //search through the item database for the item
					for itemresult in itemcursor {
						if let Ok(actualitem) = itemresult //if the item is valid
						{
							let _bson = mongodb::to_bson(&actualitem).unwrap();
				                        let _json = serde_json::ser::to_string(&_bson).unwrap();
			        	                doc_list.push_str(&_json); //add the item to the output string
							doc_list.push_str(",\n\t");
						}
					}
				}
			}
		}
	}
	if doc_list.len() <= 3
	{
		return String::from("No entries found");
	}
	//pop off last three unneeded characters
	doc_list.pop();
	doc_list.pop();
	doc_list.pop();
	doc_list.push_str("\n]");
	return doc_list;
}


#[get("/table?<table>")] 
pub fn get_table(conn: LogsDbConn, table: u32) -> String {
	println!("{}", table);
	let coll = conn.collection("orders");
	let filter = doc! {
		"table": table
	};
	let mut options = mongodb::coll::options::FindOptions::new();
	options.sort = Some(doc! { // sorts by most recently added
		"_id": -1
	});
	if let Ok (result) = coll.find_one(Some(filter.clone()), Some(options)) {
		if let Some(item) = result {
			let bson = mongodb::to_bson(&item).unwrap();
            let json = serde_json::ser::to_string(&bson).unwrap();
            return json;
		}
		else {
			let response = json!({
                "code": 404,
                "message": "Supplied table has no orders open or otherwise."
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

#[get("/comp")]
pub fn get_comps(_conn: LogsDbConn) -> String {
	let mut _str = String::from("[\n\t");
	let _doc = doc!{"comp": {"$gte": 0}};
	let mut _filter = mongodb::coll::options::FindOptions::new();
	_filter.projection = Some(doc!{"comp": 1, "table": 1, "empname": 1});
	let _coll = _conn.collection("orders");
	let _cursor = _coll.find(Some(_doc.clone()), Some(_filter.clone())).unwrap();
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

#[post("/", data = "<order>")]
pub fn post(conn: LogsDbConn, order: Json<Order>) -> String {
	let inner = order.into_inner(); // converts fron Json<Order> to just Order
	let doc = doc! {
		"table": inner.table,
		"id": inner.id,
		"items": inner.items,
		"status": inner.status,
		"total": inner.total,
		"tip": 0.00,
		"substitutions": inner.substitutions,
		"allergies": inner.allergies
	};	
	let existing = doc! {
		"table": inner.table,
		"status": {
			"$lt": 5 // where the status of the order is < 5 (not yet paid)
		}
	};
	let coll = conn.collection("orders");
	if let Ok(result) = coll.find_one(Some(existing.clone()), None) {
		if let Some (order) = result {
			let response = json!({ // generate a response for the user
				"code": 404,
				"message": "You cannot place an order at this time as you have an outstanding unpaid order."
			});
			return serde_json::to_string(&response).unwrap();
		}
	}
	coll.insert_one(doc.clone(), None).unwrap(); // insert the order
	let response = json!({ // generate a response for the user
		"code": 200,
		"message": "Inserted order into collection orders"
	});

	// update the order with the current date time
	let update = doc!{"$currentDate": { "placed": true}};
	coll.find_one_and_update(doc.clone(), update.clone(), None).unwrap();
	return serde_json::to_string(&response).unwrap();
}

#[post("/comp?<table>&<amount>&<employee>")]
pub fn comp (_conn: LogsDbConn, table: u32, amount: f32, employee: String) -> String {
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
			"comp": amount,
			"employee": employee
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

#[post("/?<id>&<amount>")]
pub fn apply_promotion(_conn: LogsDbConn, id: String, amount: f32) -> String {
        let cast = bson::oid::ObjectId::with_string(id.as_str());
        let coll = _conn.collection("orders");
        if let Ok(oid) = cast {
                let filter = doc! {"_id": oid};
                let _promo = doc! { "$inc": {"total": -amount} };
                if let Ok (result) = coll.find_one_and_update(filter.clone(),_promo.clone(), None) {
                        println!("Got a result");
                        if let Some(item) = result {
                                let response = json!({
                                        "code": 200,
                                        "message": "Successfully updated price for order"
                                });
                                return serde_json::to_string(&response).unwrap();
                        }
                                let response = json!({
                                "code": 404,
                                "message": "Could not find order to update."
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

#[post("/tip?<id>&<amount>")]
pub fn add_tip(_conn: LogsDbConn, id: String, amount: String) -> String {
	let amount2 : f32 = amount.parse().unwrap();
	let cast = bson::oid::ObjectId::with_string(id.as_str());
        let coll = _conn.collection("orders");
        if let Ok(oid) = cast {
            let filter = doc! {"_id": oid};
            let _promo = doc! { "$set": {"tip": amount2} };
            if let Ok (result) = coll.find_one_and_update(filter.clone(),_promo.clone(), None) {
                if let Some(item) = result {
                    let response = json!({
                        "code": 200,
                        "message": "Successfully updated tip for order"
                    });
                    return serde_json::to_string(&response).unwrap();
                }
                else {
                	let response = json!({
                		"code": 404,
                		"message": "Could not find order to add tip."
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
