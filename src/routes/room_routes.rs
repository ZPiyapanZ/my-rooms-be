use crate::config::auth::staff_jwt_secret;
use crate::handlers::room_handler::{
    create_room_handler, create_room_type_handler, get_rooms_with_pagination_handler, update_room_by_id_handler, update_room_type_by_id_handler
};
use crate::middlewares::auth::JwtMiddleware;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .wrap(JwtMiddleware::new(staff_jwt_secret()))
            .route("/create", web::post().to(create_room_handler))
            .route("{id}", web::put().to(update_room_by_id_handler))
            .route("", web::post().to(get_rooms_with_pagination_handler)),
    ).service(
        web::scope("/room-types")
        .wrap(JwtMiddleware::new(staff_jwt_secret()))
        .route("/create", web::post().to(create_room_type_handler))
        .route("{id}", web::put().to(update_room_type_by_id_handler))
    );
}
