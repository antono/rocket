extern crate diesel;

use diesel::prelude::*;

// pub struct Timestamp;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    // pub created_at: Timestamp,
    // pub updated_at: Timestamp,
}
