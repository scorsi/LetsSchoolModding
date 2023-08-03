#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modloader;
mod game;
mod utils;
mod state;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(state::State {
            game_path: game::DEFAULT_GAME_PATH.to_string().into()
        })
        .invoke_handler(tauri::generate_handler![
            game::check_game,
            modloader::download_modloader,
            modloader::clean_download_modloader,
            modloader::install_modloader,
            modloader::check_modloader
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
