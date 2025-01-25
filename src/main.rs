mod config;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use actix_web::{web, App, HttpServer};
use config::database::create_connection;
use dotenv::dotenv;

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
