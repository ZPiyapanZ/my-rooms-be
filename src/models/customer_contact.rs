use crate::schema::{customer_contacts};
use chrono::{DateTime, Utc, NaiveDate};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CustomerContact {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub phone_number: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "customer_contacts"]
pub struct  NewCustomerContact<'a> {
    pub full_name: &'a String,
    pub email: &'a String,
    pub phone_number: &'a String,
    pub created_at: &'a DateTime<Utc>,
    pub updated_at: &'a DateTime<Utc>,
}

#[derive(AsChangeset)]
#[table_name = "customer_contacts"]
pub struct  UpdateCustomerContact<'a> {
    pub full_name: &'a String,
    pub email: &'a String,
    pub phone_number: &'a String,
    pub updated_at: &'a DateTime<Utc>,
}