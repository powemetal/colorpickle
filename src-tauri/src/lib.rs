mod color_pick;
pub mod stockage;
pub mod commandes;
pub mod palettes;

use crate::color_pick::{
    cancel_color_pick, confirm_color_pick, read_pixel_at, start_color_pick,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commandes::sauvegarder,
            commandes::modifier_palette_nom,
            commandes::creer_palette,
            commandes::supprimer_palette,
            commandes::recuperer_palettes,
            commandes::supprimer_couleur,
            commandes::ajouter_couleur,
            commandes::modifier_couleur_nom,
            start_color_pick,
            read_pixel_at,
            confirm_color_pick,
            cancel_color_pick
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
