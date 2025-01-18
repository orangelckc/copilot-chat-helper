#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::Connection;
use serde_json::Value;
use std::path::PathBuf;

#[derive(serde::Serialize)]
struct Chat {
    question: String,
    answer: String,
}

#[tauri::command]
fn read_workspace(path: &str) -> Result<String, String> {
    // 转换路径字符串为 PathBuf
    let workspace_path = PathBuf::from(path);

    // 检查文件是否存在
    if !workspace_path.exists() {
        return Err(format!("工作区文件不存在: {:?}", workspace_path));
    }

    // 连接SQLite数据库
    let conn = Connection::open(&workspace_path).map_err(|e| format!("无法打开数据库: {}", e))?;

    // 查询聊天历史记录
    let mut stmt = conn
        .prepare("SELECT value FROM ItemTable WHERE key = 'interactive.sessions';")
        .map_err(|e| format!("准备查询失败: {}", e))?;

    let mut chats = Vec::new();

    let rows = stmt
        .query_map([], |row| {
            let value: String = row.get(0)?;
            Ok(value)
        })
        .map_err(|e| format!("查询数据失败: {}", e))?;

    for row in rows {
        if let Ok(value) = row {
            if let Ok(data) = serde_json::from_str::<Value>(&value) {
                // 检查数据结构是否符合预期
                if let Some(first_item) = data.as_array().and_then(|arr| arr.get(0)) {
                    if let Some(requests) = first_item.get("requests").and_then(|v| v.as_array()) {
                        for chat in requests {
                            // 获取问题和回答
                            let question = chat
                                .get("message")
                                .and_then(|m| m.get("text"))
                                .and_then(|t| t.as_str());

                            let answer = chat
                                .get("response")
                                .and_then(|r| r.as_array())
                                .and_then(|arr| arr.get(0))
                                .and_then(|v| v.get("value"))
                                .and_then(|v| v.as_str());

                            // 如果问题和回答都存在，添加到聊天记录中
                            if let (Some(q), Some(a)) = (question, answer) {
                                chats.push(Chat {
                                    question: q.to_string(),
                                    answer: a.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // 将结果转换为JSON字符串
    serde_json::to_string(&chats).map_err(|e| format!("序列化数据失败: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![read_workspace])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
