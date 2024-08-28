-- Add up migration script here
create table users{
    id int primary key not null,
    open_id text not null,
    session_key text not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp,
};

create unique index users_open_id_index on users(open_id);