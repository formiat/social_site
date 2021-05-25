table! {
    rooms (id) {
        id -> Int4,
        user_1_id -> Int4,
        user_2_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        password_hash -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    rooms,
    users,
);
