use std::path::Path;
use crate::state::State;

pub const DEFAULT_GAME_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Let's School";

fn validate_game_path(path: &str) -> Result<(), String> {
    let game_path = Path::new(path);
    if game_path.exists() {
        Ok(())
    } else {
        Err("Game path not found".to_string())
    }
}

#[tauri::command]
pub fn check_game(mut game_path: &str, state: tauri::State<State>) -> Result<bool, String> {
    if game_path == "" {
        game_path = DEFAULT_GAME_PATH;
    }

    if (validate_game_path(game_path)).is_ok() {
        let mut p = state.game_path.lock().unwrap();
        *p = game_path.to_string().into();
        Ok(true)
    } else {
        Err("Game path not found".to_string())
    }
}
