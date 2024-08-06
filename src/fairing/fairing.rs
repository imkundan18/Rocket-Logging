 use rocket::fairing::Fairing;
 use rocket::fairing::Info;
 use rocket::fairing::Kind;
 use rocket::request::Request;
 use rocket::response::Response; 
pub struct Logger;

 #[rocket::async_trait]
impl Fairing for Logger {
   fn info(&self) -> Info {
       Info {
           name: "Logger",
           kind: Kind::Request | Kind::Response,
       }
   }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::Data<'_>) {
        println!("Received request: {} , {}", request.uri(), self.info().name);
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
         println!("Sending response: {}, {}", response.status(),self.info().kind);
    }
 }
