use std::sync::{Arc, Mutex};
use actix_web::web::{self, Json};
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web::get};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct SignupInput {
    pub username: String,
    pub password: String
}


#[derive(Serialize, Deserialize)]
struct SignupResponse {
    message: String
}

#[post("/signup")]
async fn sign_up(body: Json<SignupInput>, app_state: web::Data<AppState>) -> impl Responder {
    println!("{}", body.username);
    println!("{}", body.password);
    println!("{})", app_state.users.len());

    HttpResponse::Ok().body("Hello World!")
}

struct AppState {
    user_index: Mutex<u32>,
    users: Mutex<Vec<User>>
}

struct User {
    pub id: u32,
    pub username: String,
    pub password: String
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        users: Mutex::new(vec![]),
        user_index: Mutex::new(0)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(sign_up)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
