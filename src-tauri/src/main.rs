// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, read_dir},
    io::{self, Error},
    path::PathBuf,
};

use filedata::FileData;
use homedir::my_home;
use serde::{Deserialize, Serialize};
use tauri::Manager;

mod filedata;

#[derive(Serialize, Deserialize)]
struct FileManagerData {
    current_location: FileData,
    current_files: Vec<FileData>,
    searched_term: String,
}
impl FileManagerData {
    fn new(file_path: PathBuf) -> Result<FileManagerData, io::Error> {
        let file_name = file_path.file_name().ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Starting file location has no name!",
        ))?;
        let file = fs::File::open(file_path.clone())?;
        let current_location = FileData::make_from_file(file_name.into(), file_path, file)?;

        Ok(FileManagerData {
            current_location,
            current_files: Vec::new(),
            searched_term: String::new(),
        })
    }
}
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

    let init_dir_path = get_init_dir_path();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let file_manager_data = FileManagerData::new(init_dir_path);
            app.manage(file_manager_data);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//TODO: Command line args for initial location
fn get_init_dir_path() -> PathBuf {
    my_home().unwrap().unwrap()
}
