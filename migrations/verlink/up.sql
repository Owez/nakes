CREATE TABLE verlink (
    id INTEGER PRIMARY KEY REFERENCES package(id),
    depends_on_id INTEGER PRIMARY KEY REFERENCES package(id),
);