#![feature(proc_macro_hygiene, plugin, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;

use dotenv::dotenv;
use routes::*;
use std::env;

mod db;
mod models;
mod routes;
mod schema;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![static_files::index, static_files::all,])
        .mount(
            "/api/v1/",
            routes![
                user::index,
                user::new,
                user::show,
                user::update_by_id,
                user::delete_by_id,
                room::index,
                room::new,
                room::show,
                room::update_by_id,
                room::delete_by_id,
            ],
        )
}

fn main() {
    rocket().launch();
}
