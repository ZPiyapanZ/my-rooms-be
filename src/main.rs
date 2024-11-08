mod config;
mod handlers;
mod models;
mod routes;
mod services;
mod schema;
mod utils;
mod middlewares;

use actix_web::{App, HttpServer, web};
use config::database::create_connection;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = create_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
