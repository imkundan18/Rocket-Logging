use crate::database::student_mdb::StudentDB;
use crate::models::student_model::Student;
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_bson};
use mongodb::results::{InsertOneResult, UpdateResult};
use rocket::http::Status;
use rocket::{ get, post, put};
use rocket::{serde::json::Json, State};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info,warn,debug,error};
use crate::models::student_model::LogData;
use mongodb::bson::DateTime;
use chrono::Local;
use std::time::SystemTime;

#[post("/insert", data = "<student>")]
pub async fn create(
    student: Json<Student>,
    db: &State<Arc<Mutex<StudentDB>>>
) -> Result<Json<InsertOneResult>, Status> {

    info!(target:"Student Data","MongoDb {student:?}");
    log_to_mongodb(&db, "INFO", &format!("MongoDb {:?}", student)).await;

    let students = student.into_inner();
    if students.name.is_empty() {
        warn!("Student name is empty, returning NotAcceptable status");
        log_to_mongodb(&db, "WARN", "Student name is empty").await;
        return Err(Status::NotAcceptable);
    }
    let db_lock = db.inner().lock().await;
    let result = db_lock.student_collection.insert_one(students, None).await;
    match result {
        Ok(res) => {
            info!("Successfully inserted student: {:?}", res);
            log_to_mongodb(&db, "info", "Student insert sucessfully").await;

            Ok(Json(res))
        }
        Err(err) => //Err(Status::NoContent),
        {
            error!("Failed to insert student: {:?}", err);
            log_to_mongodb(&db, "info", "Student insert sucessfully").await;
            Err(Status::NoContent)
        }
    }
}

#[get("/display/all")]
pub async fn display_all(db: &State<Arc<Mutex<StudentDB>>>) -> Result<Json<Vec<Student>>, Status> {
    // let mut result=match db.student_collection.find(None, None).await{
    //     Ok(result)=>result,
    //     Err(_)=>return Err(Status::NoContent),
    // };
    let db_lock = db.inner().lock().await;
    let mut result = db_lock
        .student_collection
        .find(None, None)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let mut students = Vec::new();
    while let Some(res) = result.next().await {
        debug!("Student {:?}",res);
        match res {
            Ok(rest) => students.push(rest),
            Err(_) => return Err(Status::BadRequest),
        }
        
    }
    Ok(Json(students))
}

#[put("/update/<id>", data = "<value>")]
pub async fn update(
    id: String,
    value: Json<Student>,
    db: &State<Arc<Mutex<StudentDB>>>,
) -> Result<Json<UpdateResult>, Status> {
    let new_id = id;
    if new_id.is_empty() {
        return Err(Status::BadRequest);
    }
    let new_value = Student {
        id: Some(ObjectId::parse_str(&new_id).unwrap()),
        name: value.name.to_owned(),
        age: value.age.to_owned(),
        marks: value.marks.to_owned(),
    };
    let b_data = to_bson(&new_value).unwrap();
    let filter = doc! {"$set":b_data};
    let obj_id = ObjectId::parse_str(&new_id).unwrap();
    let filter_id = doc! {"_id":obj_id};

    let db_lock = db.inner().lock().await;
    let result = db_lock
        .student_collection
        .update_one(filter_id, filter, None)
        .await;
    match result {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::BadRequest),
    }
}


//Log Collector
async fn log_to_mongodb(db: &State<Arc<Mutex<StudentDB>>>, level: &str, message: &str) {
    let log_entry = LogData {
        level: level.to_string(),
        message: message.to_string(),
        timestamp: DateTime::from(SystemTime::from(Local::now())),
    };

    let db_lock = db.inner().lock().await;
    let _ = db_lock.log_collection.insert_one(log_entry, None).await;
}

