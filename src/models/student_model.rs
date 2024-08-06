use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use mongodb::bson::DateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub age: i32,
    pub marks:f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogData {
    pub level: String,
    pub message: String,
    pub timestamp: DateTime,
}