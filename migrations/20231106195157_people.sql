-- Add migration script here
create table if not exists people
(
    id          integer primary key not null,
    name        text not null
);
