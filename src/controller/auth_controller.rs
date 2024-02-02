use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

pub fn auth_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_admin);
}

#[get("/auth")]
async fn hello_admin(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello auth controller!")
}