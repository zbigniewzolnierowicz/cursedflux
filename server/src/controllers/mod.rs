use actix_web::web;

mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").configure(user::config));
}