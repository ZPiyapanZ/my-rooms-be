use crate::config::database::DbPool;
use crate::models::staff::{CreateStaffRequest, LoginRequest, UpdateStaffRequest};
use crate::services::staff_service::{authenticate_staff, create_staff, update_staff_by_id};
use crate::utils::response::StandardResponse;
use actix_web::{web, HttpResponse};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;
use serde_json::json;

pub async fn create_staff_handler(
    pool: web::Data<DbPool>,
    req: web::Json<CreateStaffRequest>,
) -> HttpResponse {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                "Failed to get DB connection.",
            ))
        }
    };

    match create_staff(&mut conn, &req) {
        Ok(_) => HttpResponse::Created().json(StandardResponse::<()>::success(
            "Staff created successfully.",
        )),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => HttpResponse::Conflict()
            .json(StandardResponse::<()>::error("Staff email already exists.")),
        Err(_) => HttpResponse::BadRequest()
            .json(StandardResponse::<()>::error("Failed to create staff.")),
    }
}

pub async fn update_staff_by_id_handler(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    mut req: web::Json<UpdateStaffRequest>,
) -> HttpResponse {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                "Failed to get DB connection.",
            ))
        }
    };
    let id = path.into_inner();

    match update_staff_by_id(&mut conn, id, &mut req) {
        Ok(_) => HttpResponse::Ok().json(StandardResponse::<()>::success(
            "Staff updated successfully.",
        )),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().json(StandardResponse::<()>::error("Staff not found."))
        }
        Err(_) => HttpResponse::InternalServerError()
            .json(StandardResponse::<()>::error("Failed to update staff.")),
    }
}

pub async fn login_staff_handler(
    pool: web::Data<DbPool>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                "Failed to get DB connection.",
            ))
        }
    };

    match authenticate_staff(&mut conn, &req.email, &req.password) {
        Ok(token) => HttpResponse::Ok().json(StandardResponse::success_with_data(
            json!({ "token": token }),
            "Success",
        )),
        Err(msg) => HttpResponse::Unauthorized().json(StandardResponse::<()>::error(&msg)),
    }
}
