mod controllers;

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{HttpServer, App, middleware::Logger};
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(controllers::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}