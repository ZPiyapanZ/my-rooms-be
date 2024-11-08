use crate::config::database::DbPool;
use crate::models::reservation::CreateOrUpdateReservationRequest;
use crate::services::reservation_service::{create_reservation, get_reservations_with_pagination, update_reservation_by_id};
use crate::utils::common::{AppError, PaginationParams};
use crate::utils::response::StandardResponse;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub async fn create_reservation_handler(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateOrUpdateReservationRequest>,
) -> HttpResponse {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(StandardResponse::<()>::error(
                "Failed to get DB connection.",
            ))
        }
    };

    let staff_id = req.extensions().get::<i32>().unwrap().clone();

    match create_reservation(&mut conn, &body, staff_id) {
        Ok(_) => HttpResponse::Created().json(StandardResponse::<()>::success(
            "Reservation created successfully.",
        )),
        Err(AppError::BadRequest(msg)) => {
            return HttpResponse::BadRequest().json(StandardResponse::<()>::error(&msg))
        }
        Err(err) => {
            println!("Error: {:#?}", err);
            return HttpResponse::BadRequest().json(StandardResponse::<()>::error(
                "Failed to create reservation.",
            ));
        }
    }
}

pub async fn update_reservation_by_id_handler(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateOrUpdateReservationRequest>,
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
    let staff_id = req.extensions().get::<i32>().unwrap().clone();
    
    match update_reservation_by_id(&mut conn, id,&body, staff_id) {
        Ok(_) => HttpResponse::Created().json(StandardResponse::<()>::success(
            "Reservation updated successfully.",
        )),
        Err(AppError::BadRequest(msg)) => {
            return HttpResponse::BadRequest().json(StandardResponse::<()>::error(&msg))
        }
        Err(AppError::DatabaseError(diesel::result::Error::NotFound)) => {
            HttpResponse::NotFound().json(StandardResponse::<()>::error("Reservation not found."))
        }
        Err(err) => {
            println!("Error: {:#?}", err);
            return HttpResponse::BadRequest().json(StandardResponse::<()>::error(
                "Failed to updated reservation.",
            ));
        }
    }
}

pub async fn get_reservations_with_pagination_handler(
    pool: web::Data<DbPool>,
    params: web::Query<PaginationParams>,
) -> HttpResponse {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError()
        .json(StandardResponse::<()>::error("Failed to get DB connection.")),
    };
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    match get_reservations_with_pagination(&mut conn, page, page_size) {
        Ok((data, meta)) => HttpResponse::Ok().json(StandardResponse::success_with_pagination(
            data,
            "success",
            meta,
        )),
        Err(_) => HttpResponse::InternalServerError()
            .json(StandardResponse::<()>::error("Failed to get reservations.")),
    }
}