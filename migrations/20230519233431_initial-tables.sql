-- Add migration script here
create table domains (
  id bigserial primary key,
  name varchar(255) unique not null
);