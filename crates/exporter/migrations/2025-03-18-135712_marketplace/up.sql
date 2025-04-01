-- Your SQL goes here
CREATE TABLE marketplace (
    name TEXT NOT NULL,
    publisher TEXT NOT NULL,
    version TEXT NOT NULL,
    engine TEXT NOT NULL,
    platform TEXT NOT NULL,
    assert_url TEXT NOT NULL,
    is_prerelease BOOLEAN,
    hash TEXT NULL,
    PRIMARY KEY (name, publisher, version, engine, platform, assert_url)
)
