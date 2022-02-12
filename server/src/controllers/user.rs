use crate::extractors::errors::UserError;
use crate::models::user::{NewUserPayload, User, UserChangeset, UserLoginPayload};
use crate::utils::jwt::{IntoJwt, JwtClaims};
use crate::AppData;
use actix_web::cookie::SameSite;
use actix_web::error::BlockingError;
use actix_web::http::Cookie;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use chrono::Utc;
use diesel::result::Error;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
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
async fn user_data(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let user_jwt_cookie = req.cookie("user_jwt");
    let db = data.db.get().unwrap();

    if let Some(jwt_cookie) = user_jwt_cookie {
        let jwt = jwt_cookie.value();
        let parsed_jwt = decode::<JwtClaims>(
            jwt,
            &DecodingKey::from_secret(data.jwt_config.secret.as_bytes()),
            &Validation::new(data.jwt_config.algorithm),
        );

        let parsed_jwt = match parsed_jwt {
            Ok(claims) => claims,
            Err(_) => return Err(UserError::InternalServerError),
        };

        let user = match web::block(move || User::get_by_id(&db, parsed_jwt.claims.sub)).await {
            Ok(user) => user,
            Err(error) => return Err(map_error(error)),
        };

        return Ok(HttpResponse::Ok().json(user));
    }

    Err(UserError::UserNotLoggedIn)
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
    req: HttpRequest,
) -> Result<HttpResponse, UserError> {
    let db = data.db.get().unwrap();

    let user_jwt_cookie = req.cookie("user_jwt");

    if let Some(jwt_cookie) = user_jwt_cookie {
        let jwt = jwt_cookie.value();
        let parsed_jwt = decode::<JwtClaims>(
            jwt,
            &DecodingKey::from_secret(data.jwt_config.secret.as_bytes()),
            &Validation::new(data.jwt_config.algorithm),
        );

        let parsed_jwt = match parsed_jwt {
            Ok(claims) => claims,
            Err(_) => return Err(UserError::InternalServerError),
        };

        if let Err(error) = web::block(move || User::get_by_id(&db, parsed_jwt.claims.sub)).await {
            return Err(map_error(error));
        }

        return Ok(HttpResponse::Ok().finish());
    }

    let UserLoginPayload {
        email: payload_email,
        password: payload_password,
    } = payload.0;

    let user = match web::block(move || User::get_by_email(&db, payload_email)).await {
        Ok(user) => user,
        Err(error) => return Err(map_error(error)),
    };

    if User::check_login(user.clone(), payload_password) {
        let current_time = Utc::now();
        let user_jwt = user.into_jwt(
            current_time,
            chrono::Duration::minutes(10),
            data.jwt_config.algorithm,
            EncodingKey::from_secret(data.jwt_config.secret.as_bytes()),
        );

        let user_jwt = match user_jwt {
            Ok(jwt) => jwt,
            Err(_) => return Err(UserError::InternalServerError),
        };

        let user_cookie = Cookie::build("user_jwt", &user_jwt)
            .http_only(true)
            .max_age(time::Duration::new(10 * 60, 0))
            .same_site(SameSite::None)
            .finish();

        Ok(HttpResponse::Ok().cookie(user_cookie).finish())
    } else {
        Err(UserError::MismatchedPassword)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user_data)
        .service(register_user)
        .service(login_user);
}
