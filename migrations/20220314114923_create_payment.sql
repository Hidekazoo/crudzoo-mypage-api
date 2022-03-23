-- Add migration script here
CREATE TABLE users (
    id serial NOT NULL,
    PRIMARY KEY (id),
    email text
);

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
    user_id int Not NULL references users(id),
    amount INTEGER NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);