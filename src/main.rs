#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use std::time::Duration;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http::header, middleware::Logger};
use controller::{role_controller, user_controller};
use sqlx::postgres::{PgPool, PgPoolOptions};

mod config;
mod controller;
mod dto;
mod entity;
mod error;
mod query;
mod service;
mod utils;

pub struct PostgresState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let origins = std::env::var("ORIGINS").expect("ORIGINS must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful !");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&origins)
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(PostgresState { db: pool.clone() }))
            .configure(role_controller::role_route_config)
            .configure(user_controller::user_route_config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .client_request_timeout(Duration::from_secs(15))
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
