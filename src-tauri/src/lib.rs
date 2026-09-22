mod color_pick;
mod stockage;
mod commandes;
mod palettes;

use crate::color_pick::color_pick::{
    cancel_color_pick, confirm_color_pick, read_pixel_at, start_color_pick,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_color_pick,
            read_pixel_at,
            confirm_color_pick,
            cancel_color_pick
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
