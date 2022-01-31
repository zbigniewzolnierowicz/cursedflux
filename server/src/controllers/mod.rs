use crate::models::user::{NewUserPayload, User, UserChangeset};
use crate::AppData;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web::error::BlockingError;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use diesel::result::Error;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use crate::extractors::UserError;

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
) -> impl Responder {
    let db = data.db.get().unwrap();

    let password_salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_ref(), &password_salt).unwrap().to_string();

    let changeset = UserChangeset {
        username: payload.0.username,
        password_hash,
        password_salt: password_salt.as_ref().to_string(),
        email: payload.0.email,
    };

    let res = web::block(move || User::create(&db, &changeset))
        .await
        .map(|user| { HttpResponse::Ok().json(user) })
        .map_err(|err| match err {
            BlockingError::Error(Error::DatabaseError(db_error_kind, _)) => UserError::from(db_error_kind),
            _ => UserError::InternalServerError,
        });
    res
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello).service(register_user);
}
