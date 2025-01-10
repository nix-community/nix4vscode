#![allow(dead_code)]
use std::sync::LazyLock;

use rusqlite::Connection;
use tokio::sync::Mutex;

pub static GLOBAL_DB: LazyLock<Db> = LazyLock::new(|| {
    let path = format!(
        "{}/{}/{}",
        std::env::var("HOME").unwrap(),
        ".cache",
        "nix4vscode"
    );
    Db::new(&path).unwrap()
});

#[derive(Debug)]
pub struct Entry {
    publisher: String,
    name: String,
    url: String,
    sha256: Option<String>,
    platform: String,
    engine: String,
}

pub struct Db {
    conn: Mutex<Connection>,
}

impl Db {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute(
            r"
CREATE TABLE IF NOT EXISTS extension (
    publisher TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT PRIMARY KEY NOT NULL,
    sha256 TEXT NULL,
    platform TEXT NOT NULL,
    engine TEXT NOT NULL
);
        ",
            (),
        )?;

        Ok(Self { conn: conn.into() })
    }

    pub async fn insert(&self, entry: &Entry) -> anyhow::Result<()> {
        self.conn.lock().await.execute(
            r"
INSERT INTO extension (publisher, name, url, platform, engine)
VALUES(?1, ?2, ?3, ?4, ?5);
        ",
            (
                &entry.publisher,
                &entry.name,
                &entry.url,
                &entry.platform,
                &entry.engine,
            ),
        )?;
        Ok(())
    }

    pub async fn update_sha256(&self, url: &str, sha256: &str) -> anyhow::Result<()> {
        self.conn.lock().await.execute(
            r"
UPDATE extension SET sha256 = ?1 WHERE url = ?2;
        ",
            (url, sha256),
        )?;
        Ok(())
    }

    pub async fn query<F>(
        &self,
        publisher: &str,
        name: &str,
        predicate: Option<F>,
    ) -> anyhow::Result<Vec<Entry>>
    where
        F: Fn(&Entry) -> bool,
    {
        let conn = self.conn.lock().await;

        let mut stmt = conn
            .prepare(
                r"
SELECT publisher, name, url, sha256, platform, engine FROM extension
WHERE publisher = :publisher AND name = :name
",
            )
            .unwrap();
        let iter = stmt.query_map(&[(":publisher", publisher), (":name", name)], |row| {
            let entry = Entry {
                publisher: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                sha256: row.get(3)?,
                platform: row.get(4)?,
                engine: row.get(5)?,
            };

            if let Some(predicate) = &predicate {
                if !predicate(&entry) {
                    return Err(rusqlite::Error::ExecuteReturnedResults);
                }
            }

            Ok(entry)
        })?;

        let mut res = vec![];

        for item in iter.flatten() {
            res.push(item)
        }

        Ok(res)
    }
}
