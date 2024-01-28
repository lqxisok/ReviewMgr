-- Your SQL goes here
CREATE TABLE reviews (
    id INTEGER NOT NULL PRIMARY KEY,
    project_id INTEGER NOT NULL,
    status BOOLEAN NOT NULL,
    reviewer TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);
