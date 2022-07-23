use dotenv::dotenv;
use std::env;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

use crate::{
    controllers::{db_controller::{current_temperature, create_coordinate}, echo, hello, manual_hello},
    repositories::MongoRepo,
};
mod controllers;
mod repositories;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("APP_HOST").expect("You must set the APP_HOST environment var!");
    let port = env::var("APP_PORT").expect("You must set the APP_PORT environment var!");

    println!("{} {}", host, port);
    println!("{}", env::var("MONGODB_URI").expect(""));

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        let app_entry = App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(echo)
            .service(current_temperature)
            .service(create_coordinate)
            .route("/hey", web::get().to(manual_hello));

        app_entry
    })
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
