// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod board;
mod agent;
mod public;
mod conversion;
mod commands;



fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commands::get_legal_moves, commands::make_move,
      commands::get_default_state, commands::get_best_move
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
