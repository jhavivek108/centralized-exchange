use actix_web::web::{self, Json};
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web::get};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct SignupInput {
    pub username: String,
    pub password: String
}


#[post("/signup")]
async fn sign_up(body: Json<SignupInput>, app_state: web::Data<AppState>) -> impl Responder {
    println!("{}", body.username);
    println!("{}", body.password);

    HttpResponse::Ok().body("Hi!")
}

struct AppState {
    users: Vec<User>
}

struct User {
    pub id: u32,
    pub username: String,
    pub password: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users: Vec<User>= vec![];
    let app_state: AppState = AppState {
        users : vec![],
    };

    HttpServer::new(|| {
        App::new()
         .app_data(app_state)
         .service(sign_up)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
