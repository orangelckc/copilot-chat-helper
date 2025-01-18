use std::path::Path;
use whoami;

#[tauri::command]
fn check_workspace_storage_path() -> bool {
    let workspace_path = format!(
        "/Users/{}/Library/Application Support/Code/User/workspaceStorage",
        whoami::username()
    );

    Path::new(&workspace_path).exists()
}

#[tauri::command]
fn export_chat_records() -> bool {
    let db_file = String::from("state.vscdb");
    let db_path = format!(
        "/Users/{}/Library/Application Support/Code/User/workspaceStorage/{}",
        whoami::username(),
        db_file
    );

    if !Path::new(&db_path).exists() {
        return false;
    }

    // 读取sqlite文件...
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            check_workspace_storage_path,
            export_chat_records
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
