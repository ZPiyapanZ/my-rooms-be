use crate::schema::staff;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub position: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "staff"]
pub struct NewStaff<'a> {
    pub name: &'a String,
    pub email: &'a String,
    pub password: &'a String,
    pub position: &'a String,
    pub created_at: &'a DateTime<Utc>,
    pub updated_at: &'a DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateStaffRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub position: String,
}

#[derive(AsChangeset)]
#[table_name = "staff"]
pub struct UpdateStaffData {
    pub name: String,
    pub password: Option<String>,
    pub position: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UpdateStaffRequest {
    pub name: String,
    pub password: Option<String>,
    pub position: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
