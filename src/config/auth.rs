use std::env;

pub fn staff_jwt_secret() -> String {
    env::var("STAFF_JWT_SECRET").expect("STAFF_JWT_SECRET must be set")
}