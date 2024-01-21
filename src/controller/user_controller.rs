use actix_web::{web, HttpResponse, get, Responder};

use crate::PostgresState;

pub fn user_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_user);
}

#[get("/")]
async fn hello_user(data: web::Data<PostgresState>) -> impl Responder {
    HttpResponse::Ok().body("Hello user controller!")
}