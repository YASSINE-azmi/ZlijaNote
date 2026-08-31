mod application;
mod commands;
mod domain;
mod error;
mod infrastructure;

#[tauri::command]
fn get_backend_status() -> String {
    "Backend is running".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_backend_status,
            commands::app_config_commands::load_app_config,
            commands::workspace_commands::create_workspace,
            commands::workspace_commands::open_workspace
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
