use crate::models::customer_contact::{CustomerContact, NewCustomerContact, UpdateCustomerContact};
use crate::models::reservation::{
    CreateOrUpdateReservationRequest, NewReservation, Reservation, ReservationWithJoin,
    UpdateReservation,
};
use crate::models::room::{Room, RoomTypes, RoomWithType};
use crate::schema::{customer_contacts, reservations, room_types, rooms};
use crate::utils::common::AppError;
use crate::utils::response::PaginationMeta;
use chrono::{NaiveDate, Utc};
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Bool;

fn check_overlapping(
    new_check_in_date: NaiveDate,
    new_check_out_date: NaiveDate,
    room_id: i32,
    reservation_id: Option<i32>,
    conn: &mut PgConnection,
) -> Result<i64, diesel::result::Error> {
    let mut query = reservations::table.into_boxed();

    // TODO: Changed logic because SQL Injection
    query = query
        .filter(reservations::room_id.eq(room_id))
        .filter(sql::<Bool>(&format!(
            "('{new_check_in_date}' BETWEEN check_in_date AND check_out_date) OR
    ('{new_check_out_date}' BETWEEN check_in_date AND check_out_date) OR
    (check_in_date BETWEEN '{new_check_in_date}' AND '{new_check_out_date}') OR
    (check_out_date BETWEEN '{new_check_in_date}' AND '{new_check_out_date}')
    ",
            new_check_in_date = new_check_in_date,
            new_check_out_date = new_check_out_date,
        )));

    if let Some(id) = reservation_id {
        query = query.filter(reservations::id.ne(id));
    }

    query.count().get_result::<i64>(conn)
}

pub fn create_reservation(
    conn: &mut PgConnection,
    data: &CreateOrUpdateReservationRequest,
    staff_id: i32,
) -> Result<(), AppError> {
    let now = Utc::now();

    let overlapping_count = check_overlapping(
        data.check_in_date,
        data.check_out_date,
        data.room_id,
        None,
        conn,
    )?;

    if overlapping_count > 0 {
        return Err(AppError::BadRequest(
            "Dates overlap with an existing reservation.".to_string(),
        ));
    }

    let new_customer_contact = NewCustomerContact {
        full_name: &data.full_name,
        email: &data.email,
        phone_number: &data.phone_number,
        created_at: &now,
        updated_at: &now,
    };

    let room: RoomWithType = rooms::table
        .filter(rooms::id.eq(data.room_id))
        .inner_join(room_types::table)
        .select((rooms::id, room_types::price_per_night))
        .first::<RoomWithType>(conn)?;

    let customer_contact_data = diesel::insert_into(customer_contacts::table)
        .values(&new_customer_contact)
        .get_result::<CustomerContact>(conn)?;
    let diff = data
        .check_out_date
        .signed_duration_since(data.check_in_date);
    let total_price = (diff.num_days() as i32) * room.price_per_night;

    let mut new_reservation = NewReservation {
        room_id: &data.room_id,
        customer_contact_id: customer_contact_data.id,
        check_in_date: &data.check_in_date,
        check_out_date: &data.check_out_date,
        total_price,
        status: &data.status,
        created_by: Some(staff_id),
        created_at: &now,
        updated_by: Some(staff_id),
        updated_at: &now,
        confirmed_by: None,
        confirmed_at: None,
        cancelled_by: None,
        cancelled_at: None,
    };

    if data.status.as_str() == "confirmed" {
        new_reservation.confirmed_by = Some(staff_id);
        new_reservation.confirmed_at = Some(now);
    }

    let _ = diesel::insert_into(reservations::table)
        .values(&new_reservation)
        .execute(conn)?;

    Ok(())
}

pub fn update_reservation_by_id(
    conn: &mut PgConnection,
    reservation_id: i32,
    data: &CreateOrUpdateReservationRequest,
    staff_id: i32,
) -> Result<(), AppError> {
    let reservation = reservations::table
        .filter(reservations::id.eq(reservation_id))
        .first::<Reservation>(conn)?;
    let room: RoomWithType = rooms::table
        .filter(rooms::id.eq(data.room_id))
        .inner_join(room_types::table)
        .select((rooms::id, room_types::price_per_night))
        .first::<RoomWithType>(conn)?;
    let now = Utc::now();

    // Check check in or check out date is overlapping
    let overlapping_count = check_overlapping(
        data.check_in_date,
        data.check_out_date,
        data.room_id,
        Some(reservation_id),
        conn,
    )?;
    if overlapping_count > 0 {
        return Err(AppError::BadRequest(
            "Dates overlap with an existing reservation.".to_string(),
        ));
    }

    // Update customer contact
    let update_customer_contact = UpdateCustomerContact {
        full_name: &data.full_name,
        email: &data.email,
        phone_number: &data.phone_number,
        updated_at: &now,
    };
    diesel::update(
        customer_contacts::table.filter(customer_contacts::id.eq(&reservation.customer_contact_id)),
    )
    .set(update_customer_contact)
    .execute(conn)?;

    // Calculate total price
    let diff = data
        .check_out_date
        .signed_duration_since(data.check_in_date);
    let total_price = (diff.num_days() as i32) * room.price_per_night;

    // Update reservation
    let mut update_reservation = UpdateReservation {
        room_id: &data.room_id,
        check_in_date: &data.check_in_date,
        check_out_date: &data.check_out_date,
        total_price,
        status: &data.status,
        updated_by: Some(staff_id),
        updated_at: &now,
        confirmed_by: None,
        confirmed_at: None,
        cancelled_by: None,
        cancelled_at: None,
    };

    if data.status.as_str() == "confirmed" {
        update_reservation.confirmed_by = Some(staff_id);
        update_reservation.confirmed_at = Some(now);
    } else if data.status.as_str() == "cancelled" {
        update_reservation.cancelled_by = Some(staff_id);
        update_reservation.cancelled_at = Some(now);
    }

    diesel::update(reservations::table.filter(reservations::id.eq(reservation_id)))
        .set(update_reservation)
        .execute(conn)?;

    Ok(())
}

pub fn get_reservations_with_pagination(
    conn: &mut PgConnection,
    page: i64,
    page_size: i64,
) -> Result<(Vec<ReservationWithJoin>, PaginationMeta), diesel::result::Error> {
    let total_items = reservations::table.count().get_result::<i64>(conn)?;
    let total_pages = (total_items as f64 / page_size as f64).ceil() as i64;
    let offset = (page - 1) * page_size;

    let results: Vec<(
        Reservation,
        Option<Room>,
        Option<RoomTypes>,
        Option<CustomerContact>,
    )> = reservations::table
        .left_join(rooms::table.on(rooms::id.eq(reservations::room_id)))
        .left_join(room_types::table.on(room_types::id.nullable().eq(rooms::type_id.nullable())))
        .left_join(
            customer_contacts::table
                .on(customer_contacts::id.eq(reservations::customer_contact_id)),
        )
        .limit(page_size)
        .offset(offset)
        .load::<(
            Reservation,
            Option<Room>,
            Option<RoomTypes>,
            Option<CustomerContact>,
        )>(conn)?;
    // let reservations_data = reservations::table.limit(page_size).offset(offset).load::<Reservation>(conn)?;

    let formatted_results: Vec<ReservationWithJoin> = results
        .into_iter()
        .map(
            |(reservation, room, room_type, customer_contact)| ReservationWithJoin {
                reservation,
                room,
                room_type,
                customer_contact,
            },
        )
        .collect();
    let pagination_meta = PaginationMeta {
        total_items,
        total_pages,
        current_page: page,
        page_size,
    };

    Ok((formatted_results, pagination_meta))
}
