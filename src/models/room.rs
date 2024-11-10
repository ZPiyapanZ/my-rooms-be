use crate::schema::{room_types, rooms};
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Room {
    pub id: i32,
    pub room_name: String,
    pub type_id: Option<i32>,
    pub capacity: i32,
    pub is_available: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "rooms"]
pub struct NewRoom<'a> {
    pub room_name: &'a String,
    pub type_id: Option<i32>,
    pub capacity: &'a i32,
    pub is_available: &'a bool,
    pub created_by: i32,
    pub updated_by: i32,
    pub created_at: &'a DateTime<Utc>,
    pub updated_at: &'a DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateRoomRequest {
    pub room_name: String,
    pub type_id: Option<i32>,
    pub capacity: i32,
    pub is_available: bool,
}

#[derive(AsChangeset)]
#[table_name = "rooms"]
pub struct UpdateRoomData {
    pub room_name: String,
    pub type_id: Option<i32>,
    pub capacity: i32,
    pub is_available: bool,
    pub updated_by: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRoomRequest {
    pub room_name: String,
    pub type_id: Option<i32>,
    pub capacity: i32,
    pub is_available: bool,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct RoomTypes {
    pub id: i32,
    pub type_name: String,
    pub description: Option<String>,
    pub price_per_night: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

#[derive(Insertable, Queryable, Debug)]
#[table_name = "room_types"]
pub struct NewRoomTypes<'a> {
    pub type_name: &'a String,
    pub description: Option<String>,
    pub price_per_night: i32,
    pub created_at: &'a DateTime<Utc>,
    pub updated_at: &'a DateTime<Utc>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct CreateOrUpdateRoomTypesRequest {
    pub type_name: String,
    pub description: Option<String>,
    pub price_per_night: i32,
    pub room_ids: Option<Vec<i32>>,
}

#[derive(AsChangeset)]
#[table_name = "room_types"]
pub struct UpdateRoomTypeData {
    pub type_name: String,
    pub description: Option<String>,
    pub price_per_night: i32,
    pub updated_at: DateTime<Utc>,
    pub updated_by: i32,
}

#[derive(Queryable, Serialize, Debug)]
pub struct RoomWithType {
    pub id: i32,
    pub price_per_night: i32,
}
