// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod field;
use field::Field;

use tauri::Manager;

fn main() {
    let mut field = Field::create(30, 30);

    field.gen_map(4);
    field.print();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let app_handle = app.app_handle();
            std::thread::spawn(move || loop {
                app_handle.emit_all("InitialField", &serde_json::to_string(&field).unwrap()).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1))
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
