use crate::rocket_contrib;
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::LogsDbConn;

use rocket_contrib::{databases::mongodb};
use rocket_contrib::json::Json;
use mongodb::{doc, bson};

#[derive(Serialize, Deserialize)]
pub struct Survey {
        experience: String,
	experience_comment: String,
        food: String,
	food_comment: String,
	rating: String,
	rating_comment: String
}

#[get("/")]
pub fn get_all(_conn: LogsDbConn) -> String
{
 	let mut str = String::from("[\n\t");
        let _coll = _conn.collection("surveys");
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

#[post("/", data = "<survey>")]
pub fn post(_conn: LogsDbConn, survey: Json<Survey>) -> String
{
	let inner = survey.into_inner();
        //create the document to insert
        let doc = doc! {
                "experience": inner.experience,
		"experience_comment": inner.experience_comment,
                "food": inner.food,
		"food_comment": inner.food_comment,
                "rating": inner.rating,
		"rating_comment": inner.rating_comment
        };

        let _coll = _conn.collection("surveys");
        _coll.insert_one(doc, None).unwrap();
        let response = json!({
                "code": 200,
                "message": "Successfully added survey."
        });

        return serde_json::to_string(&response).unwrap();
}
