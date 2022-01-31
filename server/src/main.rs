#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rand;
extern crate argon2;

mod controllers;
mod schema;
mod models;

use env_logger::Env;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use actix_web::middleware::normalize::TrailingSlash;
use actix_web::web;
use diesel::r2d2::{ConnectionManager,PooledConnection};
use crate::diesel::r2d2;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DB = PooledConnection<ConnectionManager<PgConnection>>;

pub struct AppData {
    pub db: Arc<DbPool>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{HttpServer, App, middleware::{Logger, NormalizePath}};
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL missing.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .data(AppData { db: Arc::from(pool.clone()) })
            .service(
                web::scope("/api")
                        .service(web::scope("/user").configure(controllers::config))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}