CREATE TABLE depends (
    id INTEGER PRIMARY KEY REFERENCES package(id),
    target_id INTEGER NOT NULL,
    FOREIGN KEY(target_id) REFERENCES package(id)
);