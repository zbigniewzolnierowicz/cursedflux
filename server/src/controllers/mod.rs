use actix_web::{HttpResponse, web};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(HelloResponse { message: "Henlo!".to_string() })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api")
            .route(web::get().to(hello))
    );
}