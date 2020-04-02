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


//Building a struct to contain notifications within
#[derive(Serialize, Deserialize)]
pub struct notification
{
	//holds table id from which notif originated
	table: u32,
	//Holds type of notification (help, refill, etc)
	notif_type: u32,
	//holds details regarding refill or type of help
	#[serde(default)] details : String
}


//====================================================================================
// WIP
#[post("/", data = "<notifications>")]
pub fn post(_conn: LogsDbConn) -> &'static str
{
	
	
	
	
	
	//Establish connection to db collection for notifications
	let _coll = _conn.collection("notifications");
	//_coll.insert_one(doc!{ "
	
}



//====================================================================================

#[post("/", data = "<employee>")]
pub fn post(_conn: LogsDbConn, employee: Json<Employee>) -> String {
	let inner = employee.into_inner();
	let doc = doc!
	{
		"first_name": inner.first_name,
		"last_name": inner.last_name,
		"id": inner.id,
		"password": inner.password,
		"wage" : inner.wage,
		"phone": inner.phone,
		"position": inner.position
	};
	
	let _coll = _conn.collection("staff");
	//Does a check to ensure non-null values with minimal crash risk
	_coll.insert_one(doc, None).unwrap();
	
	let response = json!({
		"code": 200,
		"message": "Inserted order into collection orders"
	});
	
	
	return serde_json::to_string(&response).unwrap();
}
