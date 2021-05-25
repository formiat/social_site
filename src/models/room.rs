use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::rooms;
use crate::schema::rooms::dsl::rooms as all_rooms;

#[derive(Serialize, Queryable)]
pub struct Room {
    pub id: i32,
    pub user_1_id: i32,
    pub user_2_id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "rooms"]
pub struct NewRoom {
    pub user_1_id: i32,
    pub user_2_id: i32,
}

impl Room {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Room> {
        all_rooms
            .find(id)
            .load::<Room>(conn)
            .expect("Error loading room")
    }

    pub fn all(conn: &PgConnection) -> Vec<Room> {
        all_rooms
            .order(rooms::id.desc())
            .load::<Room>(conn)
            .expect("Error loading rooms")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, room: NewRoom) -> bool {
        use crate::schema::rooms::dsl::{user_1_id as u1, user_2_id as u2};
        let NewRoom {
            user_1_id,
            user_2_id,
        } = room;

        diesel::update(all_rooms.find(id))
            .set((u1.eq(user_1_id), u2.eq(user_2_id)))
            .get_result::<Room>(conn)
            .is_ok()
    }

    pub fn insert(room: NewRoom, conn: &PgConnection) -> bool {
        diesel::insert_into(rooms::table)
            .values(&room)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        return if Room::show(id, conn).is_empty() {
            false
        } else {
            diesel::delete(all_rooms.find(id))
                .execute(conn)
                .is_ok()
        }
    }
}
