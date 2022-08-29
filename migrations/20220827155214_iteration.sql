-- Add migration script here
-- Add migration script here
CREATE TABLE iteration (
  id UUID NOT NULL,
  PRIMARY KEY (id),
  start_date timestamptz NOT NULL,
  end_date timestamptz NOT NULL,
  hours int
);