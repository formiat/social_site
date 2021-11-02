drop table if exists users
drop table if exists rooms

create table users (
    id serial primary key,
    login varchar not null unique,
    password_hash varchar not null
);

create table rooms (
    id serial primary key,
    user_1_id integer references users (id) not null,
    user_2_id integer references users (id) not null
);

insert into users (login, password_hash) values ('test_user_1', 'test_passw_1'), ('test_user_2', 'test_passw_2'), ('test_user_3', 'test_passw_3');
insert into rooms (user_1_id, user_2_id) values (1, 2), (1, 3), (2, 3);
