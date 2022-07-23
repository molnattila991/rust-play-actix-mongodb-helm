use actix_web::{
    get, post,
    web::{self, Data},
    Responder,
};
use serde::ser::Impossible;
use serde_json::json;

use crate::repositories::MongoRepo;

#[get("/coordinate")]
pub async fn current_temperature(db: Data<MongoRepo>) -> impl Responder {
    let result = match db.get_item().await {
        Ok(r) => web::Json(json!({ "temperature": r })),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    result
}

#[post("/coordinate")]
pub async fn create_coordinate(db: Data<MongoRepo>) -> impl Responder {
    let result = match db.create_coordinate(47.5266398, 19.0833422).await {
        Ok(r) => web::Json(json!({ "temperature": r })),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    result
}
