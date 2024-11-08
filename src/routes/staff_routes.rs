use crate::config::auth::staff_jwt_secret;
use crate::handlers::staff_handler::{
    create_staff_handler, login_staff_handler, update_staff_by_id_handler,
};
use crate::middlewares::auth::JwtMiddleware;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/staffs")
            .route("/login", web::post().to(login_staff_handler))
            .service(
                web::scope("")
                    .wrap(JwtMiddleware::new(staff_jwt_secret()))
                    .route("/create", web::post().to(create_staff_handler))
                    .route("{id}", web::put().to(update_staff_by_id_handler)),
            ),
    );
}
