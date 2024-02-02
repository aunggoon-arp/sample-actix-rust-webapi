use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

pub fn admin_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_admin);
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Hello admin controller!"),
        (status = 401, description = "Invalid")
    ),
    security(
        ("Token" = []),
    )
)]
#[get("/admin")]
async fn hello_admin(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello admin controller!")
}