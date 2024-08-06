use mongodb::{Client,Collection};
use crate::models::student_model::Student;
use crate::models::student_model::LogData;
use std::env;
use dotenv::dotenv;

#[derive(Debug)]
pub struct StudentDB{
    pub student_collection:Collection<Student>,
    pub log_collection: Collection<LogData>,
}

impl StudentDB{
    pub async fn init_db()->Self{
        dotenv().ok();
        let client_uri=env::var("MONGO_URI").expect("mongo uri not found");
        let client=Client::with_uri_str(&client_uri).await.unwrap();
        //let client=Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let database=client.database("LoggerDB");
        let student_collection=database.collection::<Student>("personal_info");
        let log_collection = database.collection::<LogData>("log_data"); //for Storing Log Data
        StudentDB{
            student_collection,log_collection,
        }
    }
}