use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub login: String,
    pub password_hash: String,
}

impl User {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<User> {
        all_users
            .find(id)
            .load::<User>(conn)
            .expect("Error loading user")
    }

    pub fn all(conn: &PgConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("Error loading users")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, user: NewUser) -> bool {
        use crate::schema::users::dsl::{login as l, password_hash as p};
        let NewUser {
            login,
            password_hash,
        } = user;

        diesel::update(all_users.find(id))
            .set((l.eq(login), p.eq(password_hash)))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn insert(user: NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        return if User::show(id, conn).is_empty() {
            false
        } else {
            diesel::delete(all_users.find(id))
                .execute(conn)
                .is_ok()    
        }
    }
}
