use std::env;

use bson::{doc, Document};
use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig, IndexOptions, Sphere2DIndexVersion},
    Client, Collection, IndexModel,
};

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

        let options = IndexOptions::builder()
        .unique(false)
        .sphere_2d_index_version(Sphere2DIndexVersion::V3)
        .build();

        let model = IndexModel::builder()
            .keys(doc! {"location": "2dsphere"})
            .options(options)
            .build();

        test_collection.create_index(model, None).await.expect("Error creating index");

        MongoRepo {
            col: test_collection,
        }
    }

    pub async fn create_coordinate(&self, x: f64, y: f64) -> Result<i32, Error> {
        self.col
            .insert_one(
                doc! {
                    "location": {
                        "type": "Point",
                        "coordinates": [x, y]
                    }
                },
                None,
            )
            .await?;

        Ok(0)
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
