use rocket::Config;
mod models;
mod database;
mod controller;
mod routes;
pub mod fairing;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::database::student_mdb::StudentDB;
use dotenv::dotenv;
use chrono::Local;
use std::env;
use log::LevelFilter;
use env_logger::Builder;
use std::fs::File;
use std::io::Write;


#[rocket::main]
async fn main()->Result<(),rocket::Error>{
    
    //DataBase Connection
    let db=StudentDB::init_db().await;
    let db_m= Arc::new(Mutex::new(db));
    
    // Rocket Configuration
    let config=Config::figment().merge(("port",8000)).merge(("address","0.0.0.0"));

    //collect Log data
     dotenv().ok(); 
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let log_file_path = env::var("LOG_FILE_PATH").unwrap_or_else(|_| "log.app".to_string());
    
     // Set up file logging
     let log_file = File::create(&log_file_path).expect("Unable to create log file");

     Builder::new()
         .format(|buf, record|{
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
             writeln!(buf, "{} [{}]: {}",timestamp, record.level(), record.args())})
         .filter(None, log_level.parse().unwrap_or(LevelFilter::Info)) 
         .target(env_logger::Target::Pipe(Box::new(log_file))) 
         .write_style(env_logger::WriteStyle::Never)
         .init();
 
     // Test logging
     log::info!("Logging system initialized with level: {}", log_level);
 
    //Rocket Configuration
    let _=rocket::custom(config).manage(db_m).attach(fairing::fairing::Logger)
    .mount("/",routes::student_routes::stu_routes())
    .launch()
    .await;
    Ok(())

}

