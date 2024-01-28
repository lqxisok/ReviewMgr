-- Your SQL goes here
CREATE TABLE projects (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    tex_path TEXT NOT NULL,
    bib_path TEXT NOT NULL,
    proj_path TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
