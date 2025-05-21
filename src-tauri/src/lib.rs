mod action;
mod model;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            action::todo::get_todos,
            action::todo::add_todo,
            action::todo::toggle_todo,
            action::todo::delete_todo,
            action::weather::get_weather_for_city
        ]);

    builder
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
