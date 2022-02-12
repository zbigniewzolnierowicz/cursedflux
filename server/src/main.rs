#[macro_use]
extern crate diesel;
extern crate argon2;
extern crate dotenv;
extern crate rand;

mod controllers;
mod extractors;
mod models;
mod schema;
mod utils;

use crate::diesel::r2d2;
use crate::header::{ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN};
use crate::utils::jwt::JwtConfig;
use actix_cors::Cors;
use actix_web::middleware::{normalize::TrailingSlash, DefaultHeaders};
use actix_web::{http::header, web};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use dotenv::dotenv;
use env_logger::Env;
use jsonwebtoken::Algorithm;
use std::env;
use std::sync::Arc;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DB = PooledConnection<ConnectionManager<PgConnection>>;

pub struct AppData {
    pub db: Arc<DbPool>,
    pub jwt_config: Arc<JwtConfig>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        middleware::{Logger, NormalizePath},
        App, HttpServer,
    };
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL missing.");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT secret missing.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(
                DefaultHeaders::new()
                    .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                    .header(ACCESS_CONTROL_ALLOW_METHODS, "POST, GET, PUT, OPTIONS"),
            )
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .data(AppData {
                db: Arc::from(pool.clone()),
                jwt_config: Arc::from(JwtConfig {
                    secret: jwt_secret.clone(),
                    algorithm: Algorithm::HS512,
                }),
            })
            .service(web::scope("/api").configure(controllers::config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
