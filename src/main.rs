mod models;
mod schema;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::{Connection, PgConnection};

fn main() {
    println!("PostgreSQL Using Rust...!");

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = &mut PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("The Cornering Bible"),
        author: String::from("Keith Code"),
        published: true,
    };

    if models::Book::insert(book, conn) {
        println!("success : book added !");
    } else {
        println!("failed : book could not be added !");
    }
}
