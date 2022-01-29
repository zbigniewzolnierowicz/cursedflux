#[macro_use]
extern crate diesel;
extern crate dotenv;

mod controllers;

use env_logger::Env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::ConnectionManager;
use crate::diesel::r2d2;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppData {
    pub db: DbPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{HttpServer, App, middleware::Logger};
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
            .data(AppData { db: pool.clone() })
            .configure(controllers::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}