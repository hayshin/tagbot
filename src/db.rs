use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use crate::UserInfo;

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        // Initialize tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_tags (
                chat_id INTEGER,
                tag_name TEXT,
                user_id INTEGER,
                username TEXT,
                first_name TEXT,
                PRIMARY KEY (chat_id, tag_name, user_id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS muted_users (
                chat_id INTEGER,
                user_id INTEGER,
                username TEXT,
                first_name TEXT,
                PRIMARY KEY (chat_id, user_id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS private_users (
                user_id INTEGER PRIMARY KEY
            )",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn register_private_user(&self, user_id: u64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO private_users (user_id) VALUES (?)",
            params![user_id],
        )?;
        Ok(())
    }

    pub fn is_private_user(&self, user_id: u64) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT 1 FROM private_users WHERE user_id = ?")?;
        let exists = stmt.exists(params![user_id])?;
        Ok(exists)
    }

    pub fn join_tag(&self, chat_id: i64, tag_name: &str, user: &UserInfo) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "INSERT OR IGNORE INTO user_tags (chat_id, tag_name, user_id, username, first_name)
             VALUES (?, ?, ?, ?, ?)",
        )?;
        let changed = stmt.execute(params![
            chat_id,
            tag_name,
            user.id,
            user.username,
            user.first_name
        ])?;
        Ok(changed > 0)
    }

    pub fn leave_tag(&self, chat_id: i64, tag_name: &str, user_id: u64) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let changed = conn.execute(
            "DELETE FROM user_tags WHERE chat_id = ? AND tag_name = ? AND user_id = ?",
            params![chat_id, tag_name, user_id],
        )?;
        Ok(changed > 0)
    }

    pub fn mute_user(&self, chat_id: i64, user: &UserInfo) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "INSERT OR IGNORE INTO muted_users (chat_id, user_id, username, first_name)
             VALUES (?, ?, ?, ?)",
        )?;
        let changed = stmt.execute(params![
            chat_id,
            user.id,
            user.username,
            user.first_name
        ])?;
        Ok(changed > 0)
    }

    pub fn get_tag_users(&self, chat_id: i64, tag_name: &str) -> Result<Vec<UserInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT user_id, username, first_name FROM user_tags
             WHERE chat_id = ? AND tag_name = ?",
        )?;
        let rows = stmt.query_map(params![chat_id, tag_name], |row| {
            Ok(UserInfo {
                id: row.get(0)?,
                username: row.get(1)?,
                first_name: row.get(2)?,
            })
        })?;

        let mut users = Vec::new();
        for user in rows {
            users.push(user?);
        }
        Ok(users)
    }

    pub fn get_all_non_muted_users(&self, chat_id: i64) -> Result<Vec<UserInfo>> {
        let conn = self.conn.lock().unwrap();

        // Subquery to find muted user IDs for this chat
        let mut stmt = conn.prepare(
            "SELECT DISTINCT user_id, username, first_name FROM user_tags
             WHERE chat_id = ? AND user_id NOT IN (
                 SELECT user_id FROM muted_users WHERE chat_id = ?
             )",
        )?;

        let rows = stmt.query_map(params![chat_id, chat_id], |row| {
            Ok(UserInfo {
                id: row.get(0)?,
                username: row.get(1)?,
                first_name: row.get(2)?,
            })
        })?;

        let mut users = Vec::new();
        for user in rows {
            users.push(user?);
        }
        Ok(users)
    }

    pub fn list_tags(&self, chat_id: i64) -> Result<Vec<(String, i64)>> {
        let conn = self.conn.lock().unwrap();
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
    }

    pub fn get_muted_count(&self, chat_id: i64) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM muted_users WHERE chat_id = ?",
            params![chat_id],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}
