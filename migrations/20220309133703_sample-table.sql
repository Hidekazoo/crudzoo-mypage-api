-- Add migration script here
CREATE TABLE sample (
    id serial,
    PRIMARY KEY (id),
    username text,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);