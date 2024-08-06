use rocket::Route;
use rocket::routes;
use crate::controller::student_controller::{create,display_all,update};              

pub fn stu_routes()->Vec<Route>{
    routes![create,display_all,update]
    
}