use crate::db::Conn as DbConn;
use rocket_contrib::json::Json;
use crate::models::room::{Room, NewRoom};
use serde_json::Value;

#[get("/room", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let room = Room::all(&conn);

    Json(json!({
        "status": 200,
        "result": room,
    }))
}

#[post("/room", format = "application/json", data = "<new_room>")]
pub fn new(conn: DbConn, new_room: Json<NewRoom>) -> Json<Value> {
    Json(json!({
        "status": Room::insert(new_room.into_inner(), &conn),
        "result": Room::all(&conn).first(),
    }))
}

#[get("/room/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = Room::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.first(),
    }))
}

#[put("/room/<id>", format = "application/json", data = "<room>")]
pub fn update_by_id(conn: DbConn, id: i32, room: Json<NewRoom>) -> Json<Value> {
    let status = if Room::update_by_id(id, &conn, room.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/room/<id>")]
pub fn delete_by_id(conn: DbConn, id: i32) -> Json<Value> {
    let status = if Room::delete_by_id(id, &conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}
