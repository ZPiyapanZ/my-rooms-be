use crate::schema::reservations;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::{
    customer_contact::CustomerContact,
    room::{Room, RoomTypes},
};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Reservation {
    pub id: i32,
    pub room_id: i32,
    pub customer_contact_id: i32,
    pub check_in_date: NaiveDate,
    pub check_out_date: NaiveDate,
    pub total_price: i32,
    pub status: String,

    pub created_by: Option<i32>,
    pub created_at: DateTime<Utc>,

    pub updated_by: Option<i32>,
    pub updated_at: DateTime<Utc>,

    pub confirmed_by: Option<i32>,
    pub confirmed_at: Option<DateTime<Utc>>,

    pub cancelled_by: Option<i32>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[table_name = "reservations"]
pub struct NewReservation<'a> {
    pub room_id: &'a i32,
    pub customer_contact_id: i32,
    pub check_in_date: &'a NaiveDate,
    pub check_out_date: &'a NaiveDate,
    pub total_price: i32,
    pub status: &'a String,

    pub created_by: Option<i32>,
    pub created_at: &'a DateTime<Utc>,

    pub updated_by: Option<i32>,
    pub updated_at: &'a DateTime<Utc>,

    pub confirmed_by: Option<i32>,
    pub confirmed_at: Option<DateTime<Utc>>,

    pub cancelled_by: Option<i32>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct CreateOrUpdateReservationRequest {
    pub room_id: i32,
    pub check_in_date: NaiveDate,
    pub check_out_date: NaiveDate,
    pub status: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(AsChangeset)]
#[table_name = "reservations"]
pub struct UpdateReservation<'a> {
    pub room_id: &'a i32,
    pub check_in_date: &'a NaiveDate,
    pub check_out_date: &'a NaiveDate,
    pub total_price: i32,
    pub status: &'a String,

    pub updated_by: Option<i32>,
    pub updated_at: &'a DateTime<Utc>,

    pub confirmed_by: Option<i32>,
    pub confirmed_at: Option<DateTime<Utc>>,

    pub cancelled_by: Option<i32>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct ReservationWithJoin {
    pub reservation: Reservation,
    pub room: Option<Room>,
    pub room_type: Option<RoomTypes>,
    pub customer_contact: Option<CustomerContact>,
}
