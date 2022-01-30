use actix_web::{HttpResponse, web, get, post, http::Error};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(HelloResponse { message: "Henlo!".to_string() })
}

#[derive(Deserialize, Serialize)]
struct NewUserPayload {
    username: String,
    password: String
}

#[post("/")]
async fn register_user(payload: web::Json<NewUserPayload>) -> HttpResponse {
    HttpResponse::Ok().json(payload.0)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(hello)
        .service(register_user);
}