-- Add migration script here
CREATE TABLE books (
     id serial NOT NULL,
     PRIMARY KEY (id),
     title text,
     created_at timestamptz NOT NULL,
     updated_at timestamptz NOT NULL
);