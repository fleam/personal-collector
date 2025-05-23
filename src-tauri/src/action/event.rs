// src/action/event.rs

use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result as SqlResult};
use std::path::{PathBuf};
use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub date: String,
    pub title: String,
    pub description: Option<String>,
    pub is_done: bool,
}

// 获取数据库路径
fn get_db_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "fleam", "personal_collector")
        .expect("Failed to create ProjectDirs");
    let mut path = proj_dirs.data_dir().to_path_buf();
    std::fs::create_dir_all(&path).expect("Failed to create data directory");
    path.push("todos.db"); // 复用 todos.db
    path
}

// 初始化数据库和 events 表
fn init_db() -> SqlResult<()> {
    let path = get_db_path();
    let conn = Connection::open(path)?;

    // 创建 events 表（如果不存在）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            is_done BOOLEAN NOT NULL DEFAULT FALSE
        )",
        [],
    )?;

    Ok(())
}

// 获取某天的所有事件
#[tauri::command]
pub async fn get_events(date: String) -> Result<Vec<Event>, String> {
    init_db().map_err(|e| e.to_string())?;

    let path = get_db_path();
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, date, title, description, is_done FROM events WHERE date = ?")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([&date], |row| {
            Ok(Event {
                id: row.get(0)?,
                date: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                is_done: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut events = Vec::new();
    for row in rows {
        events.push(row.map_err(|e| e.to_string())?);
    }

    Ok(events)
}

// 添加事件
#[tauri::command]
pub async fn add_event(title: String, description: Option<String>, date: String) -> Result<(), String> {
    init_db().map_err(|e| e.to_string())?;

    let path = get_db_path();
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO events (date, title, description, is_done) VALUES (?, ?, ?, ?)",
        (&date, &title, &description, &false),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// 更新事件状态（完成/未完成）
#[tauri::command]
pub async fn update_event_status(id: i32, is_done: bool) -> Result<(), String> {
    init_db().map_err(|e| e.to_string())?;

    let path = get_db_path();
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE events SET is_done = ? WHERE id = ?",
        (&is_done, &id),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// 删除事件
#[tauri::command]
pub async fn delete_event(id: i32) -> Result<(), String> {
    init_db().map_err(|e| e.to_string())?;

    let path = get_db_path();
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM events WHERE id = ?", [&id])
        .map_err(|e| e.to_string())?;

    Ok(())
}