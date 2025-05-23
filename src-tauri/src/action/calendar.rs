// src/action/holiday.rs

use rusqlite::{Connection, Result as SqlResult};
use std::path::PathBuf;
use directories::ProjectDirs;

fn get_db_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "fleam", "personal_collector")
        .expect("Failed to create ProjectDirs");
    let mut path = proj_dirs.data_dir().to_path_buf();
    std::fs::create_dir_all(&path).expect("Failed to create data directory");
    path.push("todos.db");
    path
}

pub fn init_holidays_table() -> SqlResult<()> {
    let path = get_db_path();
    let conn = Connection::open(path)?;

    // 创建 holidays 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS holidays (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL
        )",
        [],
    )?;

    // 插入默认节假日数据
    insert_default_holidays(&conn)?;
    Ok(())
}

fn insert_default_holidays(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch(
        "INSERT OR IGNORE INTO holidays (date, name) VALUES
        ('2025-05-01', '劳动节'),
        ('2025-05-04', '青年节'),
        ('2025-06-01', '儿童节'),
        ('2025-06-06', '端午节')"
    )
}

#[tauri::command]
pub async fn get_holidays(month: String) -> Result<Vec<(String, String)>, String> {
    init_holidays_table().map_err(|e| e.to_string())?;

    let path = get_db_path();
    let conn = Connection::open(path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(&format!("SELECT date, name FROM holidays WHERE strftime('%Y-%m', date) = '{}'", month))
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }

    Ok(result)
}