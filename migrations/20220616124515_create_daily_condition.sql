-- Add migration script here
CREATE TABLE daily_condition (
     id serial NOT NULL,
     PRIMARY KEY (id),
     weight int,
     sleep_time int,
     mental_score int,
     created_at timestamptz NOT NULL,
     updated_at timestamptz NOT NULL
);