// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use data::ForecastRoot;

pub(crate) mod data;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(_name: &str) -> Result<String, String> {
    let result = data::controller::get_data();
    // todo: handle error
    let result = result.await.unwrap();

    Ok(format!("{:?}", result))
}

#[tauri::command]
async fn fetch_data() -> Result<ForecastRoot, String> {
    let result = data::controller::get_data().await;

    match result {
        Ok(result) => Ok(result),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn fetch_temp(hour: usize) -> Result<i32, String> {
    let result = data::controller::get_data().await;

    match result {
        Ok(result) => {
            let temp = result.get_temperature(hour);
            Ok(temp)
        }
        Err(e) => Err(e.to_string()),
    }
}
#[tauri::command]
async fn fetch_current_temperature_icon() -> Result<String, String> {
    let result = data::controller::get_data().await;

    match result {
        Ok(result) => Ok(result.get_current_temperature_icon().to_string()),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, fetch_temp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
