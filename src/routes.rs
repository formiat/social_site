use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{User, NewUser};
use serde_json::Value;

#[get("/users", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let users = User::all(&conn);

    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/users", format = "application/json", data = "<new_user>")]
pub fn new(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert(new_user.into_inner(), &conn),
        "result": User::all(&conn).first(),
    }))
}

#[get("/users/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = User::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.first(),
    }))
}

#[put("/users/<id>", format = "application/json", data = "<user>")]
pub fn update_by_id(conn: DbConn, id: i32, user: Json<NewUser>) -> Json<Value> {
    let status = if User::update_by_id(id, &conn, user.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/users/<id>")]
pub fn delete_by_id(conn: DbConn, id: i32) -> Json<Value> {
    let status = if User::delete_by_id(id, &conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}
