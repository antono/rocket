#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)] 

#[macro_use] extern crate diesel;

extern crate rocket;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::*;
// use self::schema::{users};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .filter(published.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} posts", results.len());


    rocket::ignite().mount("/", routes![index]).launch();
}
