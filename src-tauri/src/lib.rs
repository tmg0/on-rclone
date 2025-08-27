use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let window = app.get_webview_window("main").expect("no main window");
            window.show().unwrap();
            window.set_focus().unwrap();
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_rclone::init())
        .plugin(tauri_plugin_otray::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
