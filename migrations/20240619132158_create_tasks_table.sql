-- Add migration script here
CREATE TABLE tasks (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    description TEXT NOT NULL,
    is_completed BOOLEAN NOT NULL
);
