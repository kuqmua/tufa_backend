-- Add migration script here
create table cats (
  id bigserial primary key,
  name varchar(255) unique not null
);