use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use crate::models::staff::{CreateStaffRequest, NewStaff, Staff, UpdateStaffData, UpdateStaffRequest};
use crate::schema::staff::dsl::*;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use serde::{Deserialize, Serialize};
use crate::config::auth::staff_jwt_secret;
use crate::models::jwt::Claims;
pub fn create_staff(
    conn: &mut PgConnection,
    new_staff: &CreateStaffRequest,
) -> Result<(), diesel::result::Error> {
    let now = Utc::now();
    let hashed_password = hash(&new_staff.password, DEFAULT_COST).expect("Failed to hash password");
    let new_staff = NewStaff {
        name: &new_staff.name,
        email: &new_staff.email,
        password: &hashed_password,
        position: &new_staff.position,
        created_at: &now,
        updated_at: &now
    };

    diesel::insert_into(staff)
        .values(&new_staff)
        .execute(conn)?;

    Ok(())
}

pub fn update_staff_by_id(
    conn: &mut PgConnection,
    staff_id: i32,
    data: & mut UpdateStaffRequest
) -> Result<(), diesel::result::Error> {
    staff.filter(id.eq(&staff_id)).first::<Staff>(conn)?;

    let now = Utc::now();

    if let Some(pwd) = data.password.take() {
        data.password = Some(hash(&pwd, DEFAULT_COST).expect("Failed to hash password"));
    }

    let updated_data =  UpdateStaffData {
        name: data.name.clone(),
        position: data.position.clone(),
        updated_at: now,
        password: data.password.clone(),
    };

    diesel::update(staff.filter(id.eq(staff_id)))
    .set(updated_data)
    .execute(conn)?;

    Ok(())
}

pub fn authenticate_staff(
    conn: &mut PgConnection,
    email_input: &str,
    password_input: &str,
) -> Result<String, String> {
    let staff_data = staff.filter(email.eq(email_input)).first::<Staff>(conn)
    .map_err(|_| "Staff not found.".to_string())?;

    if verify(password_input, &staff_data.password).map_err(|_| "Password verification failed.".to_string())? {
        generate_jwt(staff_data.id)
    } else {
        Err("Invalid email or password.".to_string())
    }
}

fn generate_jwt(staff_id: i32) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1)) 
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: staff_id.to_owned(),
        exp: expiration as usize,
    };

    let secret_key = staff_jwt_secret();
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
        .map_err(|_| "Token generation failed".to_string())
}