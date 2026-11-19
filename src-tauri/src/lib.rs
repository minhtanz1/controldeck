pub mod commands;
pub mod controller;
pub mod detector;
pub mod injection;
pub mod mapping;
pub mod profile;
pub mod server;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::get_profiles,
            commands::get_active_profile,
            commands::set_active_profile,
            commands::save_profile,
            commands::delete_profile,
            commands::get_controller_state,
            commands::get_active_window,
            commands::get_open_windows,
            commands::simulate_controller_button,
            commands::test_action,
            commands::set_mapping_enabled,
            commands::is_mapping_enabled,
            commands::export_profile,
            commands::import_profile,
            commands::get_mobile_server_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
