use crate::models::room::{
    CreateOrUpdateRoomTypesRequest, CreateRoomRequest, NewRoom, NewRoomTypes, Room, RoomTypes,
    UpdateRoomData, UpdateRoomRequest, UpdateRoomTypeData,
};
use crate::schema::room_types;
use crate::schema::rooms::dsl::*;
use crate::utils::response::PaginationMeta;
use chrono::Utc;
use diesel::prelude::*;

pub fn create_room(
    conn: &mut PgConnection,
    data: &CreateRoomRequest,
    staff_id: i32,
) -> Result<(), diesel::result::Error> {
    let now = Utc::now();

    let new_room = NewRoom {
        room_name: &data.room_name,
        capacity: &data.capacity,
        is_available: &data.is_available,
        created_at: &now,
        updated_at: &now,
        type_id: data.type_id,
        created_by: staff_id,
        updated_by: staff_id,
    };

    let _ = diesel::insert_into(rooms).values(&new_room).execute(conn)?;

    Ok(())
}

pub fn update_room_by_id(
    conn: &mut PgConnection,
    room_id: i32,
    data: &UpdateRoomRequest,
    staff_id: i32,
) -> Result<(), diesel::result::Error> {
    rooms.filter(id.eq(&room_id)).first::<Room>(conn)?;

    let now = Utc::now();
    let duplicate_name = rooms
        .filter(room_name.eq(&data.room_name))
        .filter(id.ne(&room_id))
        .first::<Room>(conn)
        .optional()?;

    if duplicate_name.is_some() {
        return Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new("Room name already exists".to_string()),
        ));
    }

    let updated_data = UpdateRoomData {
        room_name: data.room_name.clone(),
        capacity: data.capacity,
        is_available: data.is_available.clone(),
        type_id: data.type_id,
        updated_at: now,
        updated_by: staff_id,
    };

    diesel::update(rooms.filter(id.eq(room_id)))
        .set(updated_data)
        .execute(conn)?;

    Ok(())
}

pub fn get_rooms_with_pagination(
    conn: &mut PgConnection,
    page: i64,
    page_size: i64,
) -> Result<(Vec<Room>, PaginationMeta), diesel::result::Error> {
    let total_items = rooms.count().get_result::<i64>(conn)?;
    let total_pages = (total_items as f64 / page_size as f64).ceil() as i64;

    let offset = (page - 1) * page_size;
    let rooms_data = rooms.limit(page_size).offset(offset).load::<Room>(conn)?;

    let pagination_meta = PaginationMeta {
        total_items,
        total_pages,
        current_page: page,
        page_size,
    };

    Ok((rooms_data, pagination_meta))
}

pub fn create_room_type(
    conn: &mut PgConnection,
    new_room_types: &CreateOrUpdateRoomTypesRequest,
    staff_id: i32,
) -> Result<(), diesel::result::Error> {
    let now = Utc::now();

    let create_data = NewRoomTypes {
        type_name: &new_room_types.type_name,
        description: new_room_types.description.clone(),
        price_per_night: new_room_types.price_per_night,
        created_at: &now,
        updated_at: &now,
        created_by: Some(staff_id),
        updated_by: Some(staff_id),
    };

    let room_types_data = diesel::insert_into(room_types::table)
        .values(&create_data)
        .get_result::<RoomTypes>(conn)?;

    if let Some(room_ids) = &new_room_types.room_ids {
        let _ = diesel::update(rooms)
            .filter(id.eq_any(room_ids))
            .set(type_id.eq(room_types_data.id))
            .execute(conn)?;
    }

    Ok(())
}

pub fn update_room_type_by_id(
    conn: &mut PgConnection,
    room_type_id: i32,
    data: &CreateOrUpdateRoomTypesRequest,
    staff_id: i32,
) -> Result<(), diesel::result::Error> {
    let now = Utc::now();

    let updated_data = UpdateRoomTypeData {
        type_name: data.type_name.clone(),
        description: data.description.clone(),
        price_per_night: data.price_per_night,
        updated_at: now,
        updated_by: staff_id,
    };

    diesel::update(room_types::table.filter(room_types::id.eq(room_type_id)))
        .set(updated_data)
        .execute(conn)?;

    if let Some(room_ids) = &data.room_ids {
        let _ = diesel::update(rooms)
            .filter(id.eq_any(room_ids))
            .set(type_id.eq(room_type_id))
            .execute(conn)?;
    }

    Ok(())
}
