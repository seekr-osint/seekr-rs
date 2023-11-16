-- Add migration script here
create table if not exists people
(
    id          integer primary key not null,
    name        text not null
);

create table if not exists emails (
  address text primary key not null,
  provider text not null
);

create table if not exists accounts
(
    name        text primary key not null,
    password    text
);
