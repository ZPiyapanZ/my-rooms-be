use crate::config::database::DbPool;
use crate::models::room::{CreateRoomRequest, CreateOrUpdateRoomTypesRequest, UpdateRoomRequest};
use crate::services::room_service::{create_room, create_room_type, get_rooms_with_pagination, update_room_by_id, update_room_type_by_id};
use crate::utils::common::PaginationParams;
use crate::utils::response::StandardResponse;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub async fn create_room_handler(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateRoomRequest>,
) -> HttpResponse {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError()
        .json(StandardResponse::<()>::error("Failed to get DB connection.")),
    };

    let staff_id = req.extensions().get::<i32>().unwrap().clone();

    match create_room(&mut conn, &body, staff_id) {
        Ok(_) => HttpResponse::Created().json(StandardResponse::<()>::success("Room created successfully.")),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => HttpResponse::Conflict().json(StandardResponse::<()>::error("Room name already exists.")),
        Err(_) => HttpResponse::BadRequest().json(StandardResponse::<()>::error("Failed to create room.")),
    }
}

pub async fn update_room_by_id_handler(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<UpdateRoomRequest>,
) -> HttpResponse {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError()
        .json(StandardResponse::<()>::error("Failed to get DB connection.")),
    };
    let id = path.into_inner();
    let staff_id = req.extensions().get::<i32>().unwrap().clone();

    match update_room_by_id(&mut conn, id, &body, staff_id) {
        Ok(_) => {
            HttpResponse::Ok().json(StandardResponse::<()>::success("Room updated successfully."))
        }
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => HttpResponse::Conflict()
            .json(StandardResponse::<()>::error("Room name already exists.")),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().json(StandardResponse::<()>::error("Room not found."))
        }
        Err(_) => HttpResponse::InternalServerError()
            .json(StandardResponse::<()>::error("Failed to update room.")),
    }
}

pub async fn get_rooms_with_pagination_handler(
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

    match get_rooms_with_pagination(&mut conn, page, page_size) {
        Ok((data, meta)) => HttpResponse::Ok().json(StandardResponse::success_with_pagination(
            data,
            "success",
            meta,
        )),
        Err(_) => HttpResponse::InternalServerError()
            .json(StandardResponse::<()>::error("Failed to get rooms.")),
    }
}

pub async fn create_room_type_handler(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateOrUpdateRoomTypesRequest>,
) -> HttpResponse {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError()
        .json(StandardResponse::<()>::error("Failed to get DB connection.")),
    };

    let staff_id = req.extensions().get::<i32>().unwrap().clone();

    match create_room_type(&mut conn, &body, staff_id) {
        Ok(_) => HttpResponse::Created().json(StandardResponse::<()>::success("Room type created successfully.")),
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => HttpResponse::Conflict().json(StandardResponse::<()>::error("Room type name already exists.")),
        Err(_) => HttpResponse::BadRequest().json(StandardResponse::<()>::error("Failed to create room type.")),
    }
}

pub async fn update_room_type_by_id_handler(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateOrUpdateRoomTypesRequest>,
) -> HttpResponse {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => return HttpResponse::InternalServerError()
        .json(StandardResponse::<()>::error("Failed to get DB connection.")),
    };
    let id = path.into_inner();
    let staff_id = req.extensions().get::<i32>().unwrap().clone();

    match update_room_type_by_id(&mut conn, id, &body, staff_id) {
        Ok(_) => {
            HttpResponse::Ok().json(StandardResponse::<()>::success("Room type updated successfully."))
        }
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => HttpResponse::Conflict()
            .json(StandardResponse::<()>::error("Room type name already exists.")),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().json(StandardResponse::<()>::error("Room type not found."))
        }
        Err(_) => HttpResponse::InternalServerError()
            .json(StandardResponse::<()>::error("Failed to update room type.")),
    }
}
