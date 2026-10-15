-- Add migration script here
CREATE TYPE chunk AS (
    part_number INT,
    status TEXT
);

CREATE TABLE files(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL UNIQUE,
    mime TEXT NOT NULL,
    size INT NOT NULL,
    status TEXT NOT NULL,
    chunks chunk[] NOT NULL
);
