// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::read_dir;

mod filedata;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // let test = read_dir("/home/eden/Documents/").unwrap();
    // for entry in test {
    //     let entry = entry.unwrap();
    //     let glog = filedata::FileData::try_from(entry).unwrap();
    //     let glog = serde_json::json!(glog);
    //     println!("{}", glog)
    // }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
