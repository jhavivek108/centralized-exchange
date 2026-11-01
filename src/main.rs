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
    let mut users = app_state.users.lock().unwrap();
    let mut user_index = app_state.user_index.lock().unwrap();

    let user_found = users.iter().find(|u| u.username == body.username);

    if user_found.is_none() {
        *user_index += 1;

        users.push(User {
            id: user_index.clone(),
            username: body.username.clone(),
            password: body.password.clone()
        });

        println!("{}", users.len());
        drop(users);

        HttpResponse::Ok().json(SignupResponse {
            message: String::from("Successfully signed up")
        })
    } else {
        HttpResponse::Unauthorized().json(SignupResponse {
            message: String::from("User already exists")
        })
    }
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
