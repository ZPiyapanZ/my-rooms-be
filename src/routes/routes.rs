use crate::routes::{room_routes, staff_routes};
use actix_web::web;

use super::reservation_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(room_routes::config)
            .configure(staff_routes::config)
            .configure(reservation_routes::config),
    );
}
