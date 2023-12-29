create table if not exists people
(
    id          integer primary key not null,
    owner       text not null,
    name        text not null
);
