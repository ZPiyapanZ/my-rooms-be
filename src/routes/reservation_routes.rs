use crate::config::auth::staff_jwt_secret;
use crate::handlers::reservation_handler::{create_reservation_handler, get_reservations_with_pagination_handler, update_reservation_by_id_handler};
use crate::middlewares::auth::JwtMiddleware;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reservations")
            .wrap(JwtMiddleware::new(staff_jwt_secret()))
            .route("/create", web::post().to(create_reservation_handler))
            .route("{id}", web::put().to(update_reservation_by_id_handler))
            .route("", web::post().to(get_reservations_with_pagination_handler))
    );
}
