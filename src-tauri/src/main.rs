// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, read_dir, DirEntry},
    io::{self, Error},
    path::PathBuf,
    sync::Mutex,
};

use filedata::FileData;
use homedir::my_home;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

mod filedata;

struct AppState {
    file_manager_data: Mutex<FileManagerData>,
}
impl AppState {
    fn new(file_manager_data: FileManagerData) -> AppState {
        let file_manager_data = Mutex::new(file_manager_data);
        AppState { file_manager_data }
    }
}
#[derive(Serialize, Deserialize)]
struct FileManagerData {
    current_location: FileData,
    current_files: Option<Vec<FileData>>,
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

        let current_files = match current_location.file_type {
            filedata::FileTypeEnum::File => None,
            filedata::FileTypeEnum::Directory => Some(
                fs::read_dir(&current_location.full_path)?
                    .flatten()
                    .map(FileData::try_from)
                    .flatten()
                    .collect::<Vec<FileData>>(),
            ),
            filedata::FileTypeEnum::Symlink => None,
        };

        Ok(FileManagerData {
            current_location,
            current_files,
            searched_term: String::new(),
        })
    }
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_file_manager_data(state: State<AppState>) -> String {
    let file_manager_data = state
        .file_manager_data
        .lock()
        .expect("Lock should not be poisoned!");
    serde_json::to_string(&*file_manager_data).expect("File manager data should be serializable!")
}

#[tauri::command]
fn get_current_directory_data(current_location: String) -> String {
    let data = FileManagerData::new(current_location.into())
        .expect("File manager data should be creatable!");
    serde_json::to_string(&data).expect("Data should be serializable")
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
        .invoke_handler(tauri::generate_handler![
            greet,
            get_file_manager_data,
            get_current_directory_data
        ])
        .setup(|app| {
            let file_manager_data = FileManagerData::new(init_dir_path)?;
            app.manage(AppState::new(file_manager_data));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//TODO: Command line args for initial location
fn get_init_dir_path() -> PathBuf {
    my_home().unwrap().unwrap()
}
