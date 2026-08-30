mod domain;
mod error;
mod infrastructure;

fn backend_status_message() -> String {
    "Rust backend connected successfully.".to_owned()
}

#[tauri::command]
fn get_backend_status() -> String {
    backend_status_message()
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![get_backend_status])
        .run(tauri::generate_context!())
        .expect("error while running ZlijaNote");
}

#[cfg(test)]
mod tests {
    use super::backend_status_message;

    #[test]
    fn backend_status_message_reports_connection() {
        assert_eq!(
            backend_status_message(),
            "Rust backend connected successfully."
        );
    }
}
