use crate::extractors::UserError;
use crate::models::user::{NewUserPayload, User, UserChangeset, UserLoginPayload};
use crate::utils::jwt::IntoJwt;
use crate::AppData;
use actix_web::error::BlockingError;
use actix_web::{get, post, web, HttpResponse, Responder};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use chrono::Duration;
use diesel::result::Error;
use jsonwebtoken::EncodingKey;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

fn map_error(err: BlockingError<Error>) -> UserError {
    match err {
        BlockingError::Error(Error::DatabaseError(db_error_kind, _)) => {
            UserError::from(db_error_kind)
        }
        _ => UserError::InternalServerError,
    }
}

#[get("/")]
async fn hello(data: web::Data<AppData>) -> impl Responder {
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
    let NewUserPayload {
        username,
        email,
        password,
    } = payload.0;

    let password_salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_ref(), &password_salt)
        .unwrap()
        .to_string();

    let changeset = UserChangeset {
        username,
        password_hash,
        password_salt: password_salt.as_ref().to_string(),
        email,
    };

    web::block(move || User::create(&db, &changeset))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(map_error)
}

#[post("/login/")]
async fn login_user(
    data: web::Data<AppData>,
    payload: web::Json<UserLoginPayload>,
) -> Result<HttpResponse, UserError> {
    let db = data.db.get().unwrap();
    let UserLoginPayload {
        email: payload_email,
        password: payload_password,
    } = payload.0;

    let user = match web::block(move || User::get_by_email(&db, payload_email)).await {
        Ok(user) => user,
        Err(error) => return Err(map_error(error)),
    };

    if User::check_login(user.clone(), payload_password) {
        Ok(HttpResponse::Ok().body(
            user.into_jwt(
                Duration::minutes(10),
                data.jwt_config.algorithm,
                EncodingKey::from_secret(data.jwt_config.secret.as_bytes()),
            )
            .unwrap(),
        ))
    } else {
        Err(UserError::MismatchedPassword)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(register_user)
        .service(login_user);
}
