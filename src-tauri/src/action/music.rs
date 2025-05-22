// 修改后的 use 语句（移除错误引用）
use id3::Tag;
use serde::Serialize;
use std::fs;
use std::path::Path;
use tauri::async_runtime::spawn;

#[derive(Debug, Serialize)]
pub struct Music {
    pub title: String,          // 歌曲标题（ID3.title 或文件名）
    pub name: String,           // 文件名（不含扩展名）
    pub path: String,           // 文件完整路径
    pub artist: Option<String>, // 歌手
    pub album: Option<String>,  // 专辑
}

/// 提取文件名（不含扩展名）
fn extract_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

/// 提取 MP3 元信息（标题、艺术家、专辑）
fn extract_metadata(path: &Path) -> (Option<String>, Option<String>, Option<String>) {
    if let Ok(tag) = Tag::read_from_path(path) {
        let title = tag.title().map(|s| s.to_string());
        let artist = tag.artist().map(|s| s.to_string());
        let album = tag.album().map(|s| s.to_string());
        (title, artist, album)
    } else {
        (None, None, None)
    }
}

#[tauri::command]
pub async fn get_local_music_list() -> Result<Vec<Music>, String> {
    let music_dir = "D:\\Music";

    spawn(async move { scan_music_directory(music_dir) })
        .await
        .map_err(|e| e.to_string())?
}

fn scan_music_directory<P: AsRef<Path>>(dir_path: P) -> Result<Vec<Music>, String> {
    let mut music_list = Vec::new();

    for entry in fs::read_dir(dir_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("mp3") {
            let file_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("未知")
                .to_string();

            let name = extract_name(&path);
            let display_path = path.display().to_string();

            let (title, artist, album) = extract_metadata(&path);

            music_list.push(Music {
                title: title.unwrap_or_else(|| file_name.clone()),
                name,
                path: convert_to_url(&display_path), // 替换为可访问的 URL
                artist,
                album,
            });
        }
    }

    Ok(music_list)
}

fn convert_to_url(path: &str) -> String {
    let p = path.to_string();
    let p1: String = p.replace("\\", "/");
    return "http://localhost:5173/fs/".to_string() + &p1;
}