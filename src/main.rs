use actix_web::web::Json;
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web::get};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct SignupInput {
    pub username: String,
    pub password: String
}


#[post("/signup")]
async fn sign_up(body: Json<SignupInput>) -> impl Responder {
    println!("{}", body.username);
    println!("{}", body.password);
    HttpResponse::Ok().body("Hi!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
         .service(sign_up)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
