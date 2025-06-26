-- Your SQL goes here
CREATE TABLE marketplace (
    name TEXT NOT NULL,
    publisher TEXT NOT NULL,
    version TEXT NOT NULL,
    engine TEXT NOT NULL,
    platform TEXT NOT NULL,
    is_prerelease BOOLEAN NOT NULL,
    hash TEXT NULL,
    url TEXT NULL,
    PRIMARY KEY (name, publisher, version, engine, platform)
)
