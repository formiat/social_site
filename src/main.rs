#![feature(proc_macro_hygiene, plugin, const_fn, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate diesel;

extern crate dotenv;

extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;


use dotenv::dotenv;
use std::env;
use routes::*;

mod schema;
mod models;
mod db;
mod routes;


fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![
            static_files::index, static_files::all,
        ])
        .mount("/api/v1/", routes![
            user::index, user::new, user::show, user::update_by_id, user::delete_by_id,
            room::index, room::new, room::show, room::update_by_id, room::delete_by_id,
        ])
}

fn main() {
    rocket().launch();
}
