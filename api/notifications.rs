use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

//Building a struct to contain notifications within
#[derive(Serialize, Deserialize)]
pub struct Notification
{
	//holds table id from which notif originated
	table: u32,
	//Holds type of notification (help, refill, etc)
	variant: u32,
	//holds details regarding refill or type of help
	#[serde(default)] details : serde_json::Value
}

//This first get will attempt to grab a copy of all existing notifications in the collection
#[get("/")]
pub fn get_all(_conn: LogsDbConn) -> String 
{
	//Create a string that we can add more information to as we build our get
	let mut str = String::from("[\n\t");
	
	//Establish connection to db collection for notifications
	let _coll = _conn.collection("notifications");
	//Create a scroller that starts at the beginning of our notifications collection
	let cursor = _coll.find(None, None).unwrap();
	
	//For all notifications
	for result in cursor 
	{ 
		//If an item exists
		if let Ok(item) = result 
		{
			//pull the stored item and convert to a json recognizable string (?)
			let _bson = mongodb::to_bson(&item).unwrap();
			let _json = serde_json::ser::to_string(&_bson).unwrap();
			//add the json version of our notification to our message string
			str.push_str(&_json);
		}
		//add terminals between json strings (even final case as we strip terminals before return)
		str.push_str(",\n\t");
	}
	//If cursor was empty str will be of length less than 3
	if str.len() <= 3
	{
		return String::from("No entries found");
	}
	
	//Clean up our message string of any unwanted termimnals
	str.pop();
	str.pop();
	str.pop();
	str.push_str("\n]");
	
	return str;
}

// Accepts JSON-formatted Notification and inserts into database
#[post("/", data = "<notification>")]
pub fn post(_conn: LogsDbConn, notification: Json<Notification>) -> String
{
	//doc will be our json message being sent, while inner is what allows us to access the data found in our
	// notification (?)
	let inner = notification.into_inner();
	//let arr = inner.details.as_array().unwrap();
	let doc = doc!
	{
		//matches exactly to the notification struct
		"table": inner.table,
		"variant": inner.variant,
		"details": inner.details
	};
	
	//Establish connection to db collection for notifications
	let _coll = _conn.collection("notifications");
	//Attempt to send via connection, the json formatted version of the notification
	_coll.insert_one(doc, None).unwrap();

	//if post successful
	let response = json!(
	{
		"code": 200,
		"message": "Inserted notification into notifications collection"
	});
	return serde_json::to_string(&response).unwrap();
}
