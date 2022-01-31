use crate::models::user::{NewUserPayload, User, UserChangeset};
use crate::AppData;
use actix_web::{get, post, web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

#[get("/")]
async fn hello(data: web::Data<AppData>) -> HttpResponse {
    let db = data.db.get().unwrap();
    let result = web::block(move || User::get_all(&db)).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/")]
async fn register_user(
    data: web::Data<AppData>,
    payload: web::Json<NewUserPayload>,
) -> HttpResponse {
    let db = data.db.get().unwrap();
    let password_hash = web::block(move || hash(payload.0.password, DEFAULT_COST)).await.unwrap();
    let changeset = UserChangeset {
        username: payload.0.username,
        password_hash,
        email: payload.0.email,
    };
    let result = web::block(move || User::create(&db, &changeset))
        .await
        .unwrap();
    HttpResponse::Ok().json(result)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello).service(register_user);
}
