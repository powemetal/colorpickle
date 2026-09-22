use crate::{palettes::palette};
use crate::stockage;
use tauri::AppHandle;
use tauri::State;

//fonctionnalités des Palettes
#[tauri::command]
pub fn modifier_palette_nom(app: AppHandle, id: u32, nom: String) -> Result<(), String> {
    let mut palettes = stockage::charger(&app)?;
    palette::trouver_palette_mut(&mut palettes, id)?.renommer(nom)?;
    stockage::sauvegarder(&app, palettes)?;
    Ok(())
}