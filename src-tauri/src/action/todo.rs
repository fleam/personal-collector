// src/action/todo.rs

use directories::ProjectDirs;
use rusqlite::{Connection, Result as SqlResult};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

// 初始化数据库连接和表
fn init_db() -> SqlResult<Connection> {
    let proj_dirs = ProjectDirs::from("com", "fleam", "personal_collector")
        .expect("Failed to create ProjectDirs");

    let mut path: PathBuf = proj_dirs.data_dir().to_path_buf();
    std::fs::create_dir_all(&path).expect("Failed to create data directory");
    path.push("todos.db");

    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE
        )",
        [],
    )?;

    Ok(conn)
}

// 获取所有 Todo
#[tauri::command]
pub(crate) async fn get_todos() -> Result<Vec<Todo>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let conn = init_db().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, title, completed FROM todos")
            .unwrap();
        let rows = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get(2)?,
                })
            })
            .unwrap();

        let todos = rows.into_iter().filter_map(|r| r.ok()).collect::<Vec<_>>();
        Ok(todos)
    })
    .await
    .map_err(|e| e.to_string())?
}

// 添加新 Todo
#[tauri::command]
pub(crate) async fn add_todo(title: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let conn = init_db().unwrap();
        conn.execute("INSERT INTO todos (title) VALUES (?)", &[&title])
            .unwrap();
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

// 切换完成状态
#[tauri::command]
pub(crate) async fn toggle_todo(id: i32) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let conn = init_db().unwrap();
        conn.execute(
            "UPDATE todos SET completed = NOT completed WHERE id = ?",
            &[&id],
        )
        .unwrap();
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

// 删除 Todo
#[tauri::command]
pub(crate) async fn delete_todo(id: i32) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let conn = init_db().unwrap();
        conn.execute("DELETE FROM todos WHERE id = ?", &[&id])
            .unwrap();
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
