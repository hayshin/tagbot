use tokio_rusqlite::{params, Connection, Result};
use crate::models::UserInfo;

#[derive(Debug, Clone)]
pub struct TagUserInfo {
    pub info: UserInfo,
    pub is_private: bool,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path).await?;

        // Initialize tables
        conn.call(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS users (
                    user_id INTEGER PRIMARY KEY,
                    username TEXT,
                    first_name TEXT
                )",
                [],
            )?;

            conn.execute(
                "CREATE TABLE IF NOT EXISTS user_tags (
                    chat_id INTEGER,
                    tag_name TEXT,
                    user_id INTEGER,
                    PRIMARY KEY (chat_id, tag_name, user_id),
                    FOREIGN KEY (user_id) REFERENCES users (user_id)
                )",
                [],
            )?;

            conn.execute(
                "CREATE TABLE IF NOT EXISTS muted_users (
                    chat_id INTEGER,
                    user_id INTEGER,
                    mute_type TEXT DEFAULT 'all',
                    PRIMARY KEY (chat_id, user_id, mute_type),
                    FOREIGN KEY (user_id) REFERENCES users (user_id)
                )",
                [],
            )?;

            conn.execute(
                "CREATE TABLE IF NOT EXISTS private_users (
                    user_id INTEGER PRIMARY KEY,
                    FOREIGN KEY (user_id) REFERENCES users (user_id)
                )",
                [],
            )?;

            // Migration: if the table was created without mute_type, the PRIMARY KEY might be different.
            // For a simple SQLite setup, we can try to add the column if it doesn't exist.
            let _ = conn.execute("ALTER TABLE muted_users ADD COLUMN mute_type TEXT DEFAULT 'all'", []);
            Ok(())
        }).await?;

        Ok(Self { conn })
    }

    pub async fn upsert_user(&self, user: &UserInfo) -> Result<()> {
        let user = user.clone();
        self.conn.call(move |conn| {
            conn.execute(
                "INSERT OR REPLACE INTO users (user_id, username, first_name) VALUES (?, ?, ?)",
                params![user.id, user.username, user.first_name],
            )?;
            Ok(())
        }).await
    }

    pub async fn register_private_user(&self, user: &UserInfo) -> Result<()> {
        self.upsert_user(user).await?;
        let user_id = user.id;
        self.conn.call(move |conn| {
            conn.execute(
                "INSERT OR IGNORE INTO private_users (user_id) VALUES (?)",
                params![user_id],
            )?;
            Ok(())
        }).await
    }

    pub async fn join_tag(&self, chat_id: i64, tag_name: String, user: &UserInfo) -> Result<bool> {
        self.upsert_user(user).await?;
        let user_id = user.id;
        self.conn.call(move |conn| {
            let changed = conn.execute(
                "INSERT OR IGNORE INTO user_tags (chat_id, tag_name, user_id) VALUES (?, ?, ?)",
                params![chat_id, tag_name, user_id],
            )?;
            Ok(changed > 0)
        }).await
    }

    pub async fn leave_tag(&self, chat_id: i64, tag_name: String, user_id: u64) -> Result<bool> {
        self.conn.call(move |conn| {
            let changed = conn.execute(
                "DELETE FROM user_tags WHERE chat_id = ? AND tag_name = ? AND user_id = ?",
                params![chat_id, tag_name, user_id],
            )?;
            Ok(changed > 0)
        }).await
    }

    pub async fn mute_user(&self, chat_id: i64, user_id: u64, mute_type: String) -> Result<bool> {
        self.conn.call(move |conn| {
            let changed = conn.execute(
                "INSERT OR IGNORE INTO muted_users (chat_id, user_id, mute_type) VALUES (?, ?, ?)",
                params![chat_id, user_id, mute_type],
            )?;
            Ok(changed > 0)
        }).await
    }

    pub async fn unmute_user(&self, chat_id: i64, user_id: u64, mute_type: String) -> Result<bool> {
        self.conn.call(move |conn| {
            let changed = conn.execute(
                "DELETE FROM muted_users WHERE chat_id = ? AND user_id = ? AND mute_type = ?",
                params![chat_id, user_id, mute_type],
            )?;
            Ok(changed > 0)
        }).await
    }

    pub async fn get_tag_users(&self, chat_id: i64, tag_name: String, filter_mute_type: Option<String>) -> Result<Vec<TagUserInfo>> {
        self.conn.call(move |conn| {
            let mut users = Vec::new();
            if let Some(mtype) = filter_mute_type {
                let q = "SELECT u.user_id, u.username, u.first_name, (pu.user_id IS NOT NULL) as is_private
                     FROM user_tags t
                     JOIN users u ON t.user_id = u.user_id
                     LEFT JOIN private_users pu ON u.user_id = pu.user_id
                     WHERE t.chat_id = ? AND t.tag_name = ? 
                     AND u.user_id NOT IN (
                         SELECT user_id FROM muted_users 
                         WHERE chat_id = ? AND (mute_type = 'all' OR mute_type = ?)
                     )";
                let mut stmt = conn.prepare(q)?;
                let rows = stmt.query_map(params![chat_id, tag_name, chat_id, mtype], |row| {
                    Ok(TagUserInfo {
                        info: UserInfo {
                            id: row.get(0)?,
                            username: row.get(1)?,
                            first_name: row.get(2)?,
                        },
                        is_private: row.get(3)?,
                    })
                })?;
                for user in rows {
                    users.push(user?);
                }
            } else {
                let q = "SELECT u.user_id, u.username, u.first_name, (pu.user_id IS NOT NULL) as is_private
                 FROM user_tags t
                 JOIN users u ON t.user_id = u.user_id
                 LEFT JOIN private_users pu ON u.user_id = pu.user_id
                 LEFT JOIN muted_users mu ON u.user_id = mu.user_id AND t.chat_id = mu.chat_id AND mu.mute_type = 'all'
                 WHERE t.chat_id = ? AND t.tag_name = ? AND mu.user_id IS NULL";
                let mut stmt = conn.prepare(q)?;
                let rows = stmt.query_map(params![chat_id, tag_name], |row| {
                    Ok(TagUserInfo {
                        info: UserInfo {
                            id: row.get(0)?,
                            username: row.get(1)?,
                            first_name: row.get(2)?,
                        },
                        is_private: row.get(3)?,
                    })
                })?;
                for user in rows {
                    users.push(user?);
                }
            };
            Ok(users)
        }).await
    }

    pub async fn list_tags(&self, chat_id: i64) -> Result<Vec<(String, i64)>> {
        self.conn.call(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT tag_name, COUNT(user_id) FROM user_tags
                 WHERE chat_id = ? GROUP BY tag_name",
            )?;
            let rows = stmt.query_map(params![chat_id], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?;

            let mut tags = Vec::new();
            for tag in rows {
                tags.push(tag?);
            }
            Ok(tags)
        }).await
    }

    pub async fn get_muted_count(&self, chat_id: i64) -> Result<i64> {
        self.conn.call(move |conn| {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM muted_users WHERE chat_id = ? AND mute_type = 'all'",
                params![chat_id],
                |row| row.get(0),
            )?;
            Ok(count)
        }).await
    }

    pub async fn tag_exists(&self, chat_id: i64, tag_name: String) -> Result<bool> {
        self.conn.call(move |conn| {
            let mut stmt = conn.prepare("SELECT 1 FROM user_tags WHERE chat_id = ? AND tag_name = ? LIMIT 1")?;
            let exists = stmt.exists(params![chat_id, tag_name])?;
            Ok(exists)
        }).await
    }
}
