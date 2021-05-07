#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;


#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let user1 = models::NewUser {
        login: String::from("login1"),
        password_hash: String::from("123"),
    };

    let user2 = models::NewUser {
        login: String::from("login2"),
        password_hash: String::from("456"),
    };

    if models::User::insert(user1, &conn) {
        println!("Insert 1 success");
    } else {
        println!("Insert 1 failed");
    }

    if models::User::insert(user2, &conn) {
        println!("Insert 2 success");
    } else {
        println!("Insert 2 failed");
    }

    let all_users = models::User::all(&conn);
    println!("{:?}", all_users);


    rocket::ignite()
        .mount("/", routes![hello])
        .launch();
}
