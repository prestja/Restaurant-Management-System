use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

//JSON Serialized datat type to be received with coupon information
#[derive(Serialize, Deserialize)]
pub struct TimeInfo {
        empid: u32,
        first_name: String,
        last_name: String
}

#[post("/clockin", data = "<timeinfo>")]
pub fn clock_in(_conn: LogsDbConn, timeinfo: Json<TimeInfo>) -> String
{
	let inner = timeinfo.into_inner();

        //create the document to insert
        let doc = doc!
        {
                "empid": inner.empid,
		"first_name": inner.first_name,
		"last_name": inner.last_name
        };

        //open coupons collection
        let _coll = _conn.collection("timeclock");
	let _update = doc!{"$currentDate": { "clocked_in": true}, "$set": { "clocked_out": "None"}};
        //insert the new document
	let mut _upsert = mongodb::coll::options::FindOneAndUpdateOptions::new();
	_upsert.upsert = Some(true);
	if let Ok(result) = _coll.find_one_and_update(doc.clone(), _update.clone(), Some(_upsert.clone())){
                let response = json!({
                        "code": 200,
                        "message": "Successfully updated clock in time."
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

#[post("/clockout", data = "<timeinfo>")]
pub fn clock_out(_conn: LogsDbConn, timeinfo: Json<TimeInfo>) -> String
{
        let inner = timeinfo.into_inner();

        //create the document to insert
        let doc = doc!
        {
                "empid": inner.empid,
                "first_name": inner.first_name,
                "last_name": inner.last_name
        };

        //open coupons collection
        let _coll = _conn.collection("timeclock");
        let _update = doc!{"$currentDate": { "clocked_out": true}};
        //insert the new document
        let mut _upsert = mongodb::coll::options::FindOneAndUpdateOptions::new();
        _upsert.upsert = Some(true);
        if let Ok(result) = _coll.find_one_and_update(doc.clone(), _update.clone(), Some(_upsert.clone())){
                let response = json!({
                        "code": 200,
                        "message": "Successfully updated clock out time."
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

