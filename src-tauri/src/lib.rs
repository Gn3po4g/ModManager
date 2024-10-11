use std::{
    collections::BTreeSet,
    fs::{self, File},
    io::{BufRead, BufReader},
    sync::Mutex,
};
use walkdir::WalkDir;

struct EnabledMods(Mutex<BTreeSet<String>>);

#[tauri::command]
fn get_all_mods() -> BTreeSet<String> {
    let mut all_mods = BTreeSet::<String>::new();

    if let Ok(dir) = fs::read_dir("mods") {
        let mods = dir
            .flatten()
            .filter(|e| e.path().is_dir())
            .map(|e| e.path().file_name().unwrap().to_str().unwrap().to_string());

        all_mods.extend(mods);
    }

    all_mods
}

#[tauri::command]
fn get_enabled_mods(mod_list: tauri::State<EnabledMods>) -> BTreeSet<String> {
    let mut mod_list = mod_list.0.lock().unwrap();
    if let Ok(file) = File::open("mods/enabled") {
        let lines = BufReader::new(file).lines().flatten();

        mod_list.extend(lines);
    }

    mod_list.clone()
}

#[tauri::command]
fn enable_mods(mods: Vec<String>, mod_list: tauri::State<EnabledMods>) -> Result<(), String> {
    let mut mod_list = mod_list.0.lock().unwrap();
    for item in mods {
        let mod_path = format!("mods/{}", item);
        for entry in WalkDir::new(&mod_path).min_depth(1).into_iter().flatten() {
            let source = entry.path();
            let target = entry.path().strip_prefix(&mod_path).unwrap();

            if entry.file_type().is_file() {
                fs::copy(source, target).map_err(|e| e.to_string())?;
            } else if entry.file_type().is_dir() {
                fs::create_dir(target).unwrap_or_default();
            }
        }
        mod_list.insert(item);
    }
    let vec: Vec<String> = mod_list.iter().map(ToString::to_string).collect();

    Ok(fs::write("mods/enabled", vec.join("\n")).unwrap_or_default())
}

#[tauri::command]
fn disable_mods(mods: Vec<String>, mod_list: tauri::State<EnabledMods>) -> Result<(), String> {
    let mut mod_list = mod_list.0.lock().unwrap();
    for item in mods {
        let mod_path = format!("mods/{}", item);
        for entry in WalkDir::new(&mod_path).min_depth(1).into_iter().flatten() {
            let target = entry.path().strip_prefix(&mod_path).unwrap();
            if entry.file_type().is_file() {
                fs::remove_file(target).unwrap_or_default();
            } else if entry.file_type().is_dir() {
                fs::remove_dir(target).unwrap_or_default();
            }
        }
        mod_list.remove(&item);
    }
    let vec: Vec<String> = mod_list.iter().map(ToString::to_string).collect();

    Ok(fs::write("mods/enabled", vec.join("\n")).unwrap_or_default())
}

#[tauri::command]
fn show_window(window: tauri::Window) {
    window.show().unwrap_or_default();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(EnabledMods(Default::default()))
        .invoke_handler(tauri::generate_handler![
            get_all_mods,
            get_enabled_mods,
            enable_mods,
            disable_mods,
            show_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
