-- Add migration script here
-- Add migration script here
CREATE TABLE payment_type (
    id serial NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);
CREATE TABLE payment (
    id serial NOT NULL,
    PRIMARY KEY (id),
    payment_type_id  int NOT NULL references payment_type(id),
    payment INTEGER NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);