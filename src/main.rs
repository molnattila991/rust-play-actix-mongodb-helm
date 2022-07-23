use dotenv::dotenv;
use std::env;

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use bson::{doc, Document};
use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use serde_json::json;

pub struct MongoRepo {
    col: Collection<Document>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let client_uri =
        //"mongodb://adminuser:password123@localhost:27017/?retryWrites=true&w=majority";
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
        let options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await;
        let client = Client::with_options(options.unwrap());

        let test_collection: Collection<Document> =
            client.unwrap().database("test").collection("datas");

        MongoRepo {
            col: test_collection,
        }
    }

    pub async fn get_item(&self) -> Result<i32, Error> {
        let mut query_distance = self
            .col
            .find(
                doc! {
                    "location": {
                        "$near": {
                            "$geometry": {
                                "type": "Point",
                                "coordinates": [
                                    47.5266398,19.0833422
                                ]
                            },
                            "$maxDistance": 400
                        }
                    }
                },
                None,
            )
            .await?;

        let mut counter = 0;
        while query_distance.advance().await? {
            counter += 1;
            // println!("{:?}", query_distance.deserialize_current());
            // println!("");
        }

        println!("Count {}", counter);
        Ok(counter)
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/temp")]
async fn current_temperature(db: Data<MongoRepo>) -> impl Responder {
    let result = match db.get_item().await {
        Ok(r) => web::Json(json!({ "temperature": r })),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    result
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
 
    let host = env::var("APP_HOST").expect("You must set the APP_HOST environment var!");
    let port = env::var("APP_PORT").expect("You must set the APP_PORT environment var!");

    println!("{} {}", host, port);
    println!("{}", env::var("MONGODB_URI").expect(""));

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move|| {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(echo)
            .service(current_temperature)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((host, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
