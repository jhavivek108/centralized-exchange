// use std::sync::{Arc, Mutex};
// use actix_web::web::{self, Json};
// use actix_web::{App, HttpResponse, HttpServer, Responder, post, web::get};
// use serde::{Deserialize, Serialize};
use std::{collections::HashMap, os::unix::thread, sync::{Mutex, mpsc::{self, Sender}}, thread::spawn};

use actix_web::{App, HttpServer, web::{self}};

use crate::{BalanceMessage::{GetBalance, Onramp}, routes::user::{balance, deposit, onramp, sign_in, sign_up}, types::user::User};




struct AppState {
    user_index: Mutex<u32>,
    users: Mutex<Vec<User>>
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
