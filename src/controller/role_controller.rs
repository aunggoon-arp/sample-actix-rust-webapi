use actix_web::{web, HttpResponse, get, Responder};

use crate::PostgresState;

pub fn role_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_role);
}

#[get("/")]
async fn hello_role(data: web::Data<PostgresState>) -> impl Responder {
    HttpResponse::Ok().body("Hello user controller!")
}