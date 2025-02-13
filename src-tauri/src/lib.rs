use std::fs;
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;

use regex::Regex;
use tauri::{Manager, State};
use tokio::sync::Mutex;
use sysinfo::System;
use glob::glob;
use tempfile::tempfile;
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

#[derive(Default)]
struct AppState {
    exe_path: String,
    whitelist_key: String,
    nonce: String,
}

#[tauri::command]
async fn set_info(
    state: State<'_, Mutex<AppState>>,
    exe_path: &str,
    whitelist_key: &str,
    nonce: &str,
) -> Result<(), String> {
    let exe_as_path = Path::new(exe_path);

    if !exe_as_path.exists() {
        return Err("Failed to find Gorilla Tag.exe did you select the right folder?".to_string());
    }

    let mut state = state.lock().await;

    state.exe_path = exe_as_path.to_str().unwrap().to_string();
    state.whitelist_key = whitelist_key.to_string();
    state.nonce = nonce.to_string();

    Ok(())
}

fn kill_process_by_name(name: &str) -> Result<(), String> {
    for process in System::new_all().processes_by_name(OsStr::new(name)) {
        if !process.kill() {
            return Err(format!(
                "Couldn't close {name} try to close it manually and try again."
            ));
        }

        process.wait();
    }

    Ok(())
}

fn del_all_files(path: PathBuf) {
    let paths = glob(path.to_str().unwrap()).unwrap();

    for path in paths.filter_map(|path| path.ok()) {
        println!("deleting {}", path.to_str().unwrap());
        fs::remove_dir_all(path).ok();
    }
}

fn find_file_glob(path: PathBuf) -> bool {
    let paths = glob(path.to_str().unwrap()).unwrap();

    return paths.filter_map(|path| path.ok()).any(|path| path.exists());
}

// Method not found by me
// https://stackoverflow.com/a/34091380
#[tauri::command]
async fn get_game_location() -> Result<String, ()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam").unwrap();

    let value: String = cur_ver.get_value("InstallPath").unwrap();
    let steam_path = Path::new(&value);

    let library_folders_path = Path::join(steam_path, "steamapps\\libraryfolders.vdf");
    let library_folder = fs::read_to_string(library_folders_path).unwrap();

    let path_regex = Regex::new("\"path\".*?\"(.*)\"").unwrap();

    for line in library_folder.split("\n") {
        println!("scanning {}", line);

        if let Some(regex_res) = path_regex.captures(line) {
            let value = regex_res.get(1).unwrap().as_str();
            println!("{} found res", value);
            let game_path = Path::join(Path::new(value), "steamapps\\common\\Gorilla Tag");

            if game_path.exists() {
                return Ok(game_path.to_str().unwrap().to_string())
            }
        }
    };

    Err(())
}

#[tauri::command]
async fn install_bepinex(
    state: State<'_, Mutex<AppState>>,
    download_link: &str,
    key_name: &str,
    dll_name: &str,
    nonce_name: &str,
    folder_name: &str,
) -> Result<(), String> {
    kill_process_by_name("Gorilla Tag.exe")?;

    let state = state.lock().await;
    let game_path = Path::new(state.exe_path.as_str()).parent().unwrap();

    let plugins_folder = Path::join(game_path, "BepInEx\\plugins");

    let key_file_path = Path::join(&plugins_folder, "TMA\\key.txt");
    let nonce_file_path = Path::join(&plugins_folder, "TMA\\nonce.txt");

    let request = reqwest::get(download_link).await.unwrap();
    let stream = request.bytes().await.unwrap();

    // Writes bepinex to temp
    let mut tmp_file = tempfile().unwrap();
    tmp_file.write_all(&stream).unwrap();

    // Delete old tma installation
    del_all_files(Path::join(&plugins_folder, "**\\TMA"));
    del_all_files(Path::join(&plugins_folder, format!("**\\{folder_name}")));

    // Unzip bepinex
    let mut archive = zip::ZipArchive::new(tmp_file).unwrap();
    archive.extract(game_path).unwrap();

    // Write auth info to TMA folder
    let mut key_file = fs::File::create(&key_file_path).unwrap();
    key_file.write(state.whitelist_key.as_bytes()).unwrap();

    let mut nonce_file = fs::File::create(&nonce_file_path).unwrap();
    nonce_file.write(state.nonce.as_bytes()).unwrap();

    // If dir does not already exists
    let new_folder_path = Path::join(&plugins_folder, folder_name);
    if !new_folder_path.exists() {
        fs::create_dir(new_folder_path).unwrap();
    }

    fs::rename(
        Path::join(&plugins_folder, "TMA\\key.txt"),
        Path::join(&plugins_folder, format!("{folder_name}\\{key_name}.txt")),
    ).unwrap();

    fs::rename(
        Path::join(&plugins_folder, "TMA\\nonce.txt"),
        Path::join(&plugins_folder, format!("{folder_name}\\{nonce_name}.txt")),
    ).unwrap();

    fs::rename(
        Path::join(&plugins_folder, "TMA\\TooManyAccounts.dll"),
        Path::join(&plugins_folder, format!("{folder_name}\\{dll_name}.dll")),
    ).unwrap();

    // Only for trial bc trial keep TMA as folder name
    if folder_name != "TMA" {        
        fs::remove_dir_all(Path::join(&plugins_folder, "TMA")).unwrap();
    }

    Ok(())
}

#[tauri::command]
async fn has_tma_installed(state: State<'_, Mutex<AppState>>, folder_name: &str) -> Result<bool, ()> {
    let state = state.lock().await;
    let my_path = Path::new(state.exe_path.as_str()).parent().unwrap();

    let plugins_folder = Path::join(my_path, "BepInEx\\plugins");

    let folder_1 = Path::join(&plugins_folder, "**\\TMA");
    let folder_2 = Path::join(&plugins_folder, format!("**\\{folder_name}"));

    Ok(find_file_glob(folder_1) || find_file_glob(folder_2))
}

#[tauri::command]
async fn uninstall(state: State<'_, Mutex<AppState>>, folder_name: &str) -> Result<(), ()> {
    let state = state.lock().await;
    let my_path = Path::new(state.exe_path.as_str()).parent().unwrap();

    let plugins_folder = Path::join(my_path, "BepInEx\\plugins");

    del_all_files(Path::join(&plugins_folder, "**\\TMA"));
    del_all_files(Path::join(&plugins_folder, format!("**\\{folder_name}")));

    Ok(())
}

#[tauri::command]
fn close() {
    exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_info,
            install_bepinex,
            get_game_location,
            has_tma_installed,
            close,
            uninstall
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
