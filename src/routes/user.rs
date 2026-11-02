use std::{collections::HashMap, sync::mpsc};

use actix_web::{HttpResponse, Responder, get, post, web::{self, Json}};
use chrono::{Duration, Utc};
use futures::channel::oneshot;
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{AppState, BalanceMessage::Onramp, middleware::AuthUser, types::user::{BalanceResponse, Claims, DepositRequest, DespositResponse, OnRampRequest, SigninInput, SigninResponse, SignupInput, SignupResponse, User}};

#[post("/signup")]
async fn sign_up(body: Json<SignupInput>, app_state: web::Data<AppState>) -> impl Responder {
    let mut users = app_state.users.lock().unwrap();
    let mut user_index = app_state.user_index.lock().unwrap();

    let user_found = users.iter().find(|u| u.username == body.username);

    if user_found.is_none() {
        *user_index = *user_index + 1;
        users.push(User {
            id: user_index.clone(),
            username: body.username.clone(),
            password: body.password.clone()
        });

        app_state.balances_tx.send(Onramp(user_index.clone(), 0));
        
        let mut stock_balances = app_state.stock_balances.lock().unwrap();
        stock_balances.insert(user_index.clone(), HashMap::new());

        HttpResponse::Ok().json(SignupResponse {
            message: String::from("Successfully signed up")
        })
    } else {
        HttpResponse::Unauthorized().json(SignupResponse {
            message: String::from("User already")
        })
    }
}

#[post("/signin")]
pub async fn sign_in(app_state: web::Data<AppState>, body: Json<SigninInput>) -> impl Responder {
    let mut users = app_state.users.lock().unwrap();
    let user_found = users.iter().find(|u| u.username == body.username && u.password == body.password);

    if user_found.is_none() {
        return HttpResponse::Unauthorized().json(SignupResponse {
            message: String::from("Incorrect credentials")
        });
    }

    let user = user_found.unwrap();

    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        exp
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
    
    HttpResponse::Ok().json(SigninResponse {
        token
    })
}
