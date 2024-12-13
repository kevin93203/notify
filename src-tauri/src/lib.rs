// src-tauri/src/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::{Local, NaiveDateTime};
use std::sync::{Arc, Mutex};
use tauri::Emitter;
use tauri::Manager;
use tokio::time::{sleep, Duration};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct NotificationMessage {
    id: u64,
    time: String,
    message: String,
}

#[derive(Default)]
struct AppState {
    scheduled_messages: Arc<Mutex<Vec<NotificationMessage>>>,
}

impl AppState {
    // 添加方法來操作內部狀態
    fn add_message(&self, message: NotificationMessage) {
        let mut messages = self.scheduled_messages.lock().unwrap();
        messages.push(message);
    }

    fn remove_message(&self, id: u64) {
        let mut messages = self.scheduled_messages.lock().unwrap();
        messages.retain(|msg| msg.id != id);
    }
}

#[tauri::command]
async fn schedule_notification(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    message: NotificationMessage,
) -> Result<(), String> {
    let parsed_time = NaiveDateTime::parse_from_str(&message.time, "%Y-%m-%dT%H:%M")
        .map_err(|_| "無效的時間格式")?;

    let notification_message = message.clone();
    let state_clone = state.scheduled_messages.clone();

    tokio::spawn(async move {
        let duration = parsed_time.signed_duration_since(Local::now().naive_local());
        if duration.num_seconds() > 0 {
            sleep(Duration::from_secs(duration.num_seconds() as u64)).await;

            // 發送通知
            let _ = app_handle.emit("notification_", notification_message.clone());

            // 從排程中移除
            let mut messages = state_clone.lock().unwrap();
            messages.retain(|msg| msg.id != notification_message.id);
        }
    });

    // 加入排程列表
    state.add_message(message);

    Ok(())
}

#[tauri::command]
fn cancel_notification(state: tauri::State<'_, AppState>, id: u64) -> Result<(), String> {
    state.remove_message(id);
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            app.manage(AppState::default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            schedule_notification,
            cancel_notification
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
