-- Add migration script here

CREATE TABLE file(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL UNIQUE,
    mime TEXT NOT NULL,
    size INT NOT NULL,
    status TEXT NOT NULL
);
