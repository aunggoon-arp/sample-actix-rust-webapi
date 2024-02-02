use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

pub fn post_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_post);
}

#[get("/post")]
async fn hello_post(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello post controller!")
}