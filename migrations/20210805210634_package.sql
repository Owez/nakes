CREATE TABLE package (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(32) NOT NULL,
    version VARCHAR(16) NOT NULL,
    hash VARCHAR(64) UNIQUE NOT NULL,
    CONSTRAINT uq_namver UNIQUE(name, version)
);
-- int ids are easier to deal with