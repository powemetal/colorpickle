use tauri::AppHandle;

use crate::{
    palettes::{couleur::Couleur, palette::Palette, palettes::Palettes}, stockage
};

fn charger_palettes(app: &AppHandle) -> Result<Palettes, String> {
    let palettes = stockage::charger(&app)
        .map_err(|e| format!("Erreur lors de la récupération des palettes : {e}"))?;
    Ok(Palettes::new(palettes))
}

#[tauri::command]
pub fn sauvegarder(app: AppHandle) -> Result<(), String> {
    let palettes = charger_palettes(&app)?;
    stockage::sauvegarder(&app, palettes)?;
    Ok(())
}

#[tauri::command]
pub fn recuperer_palettes(app: AppHandle) -> Result<Palettes, String> {
    charger_palettes(&app)
}

#[tauri::command]
pub fn creer_palette(app: tauri::AppHandle, nom: &str) -> Result<Palette, String> {
    let mut palettes = charger_palettes(&app)?;

    let nouvelle_palette = Palette::new(nom.to_string(), vec![]);
    palettes.ajouter_palette(nouvelle_palette.clone());

    stockage::sauvegarder(&app, palettes)?;
    Ok(nouvelle_palette)
}

#[tauri::command]
pub fn supprimer_palette(app: tauri::AppHandle, id: &str) -> Result<(), String> {
    let mut palettes = charger_palettes(&app)?;

    palettes.supprimer_palette(id)?;

    stockage::sauvegarder(&app, palettes)?;
    Ok(())
}

#[tauri::command]
pub fn modifier_palette_nom(app: tauri::AppHandle, id: &str, nom: &str) -> Result<(), String> {
    let mut palettes = charger_palettes(&app)?;

    let palette = palettes.trouver_palette(id).ok_or("Cette palette n'existe pas.")?;
    palette.set_nom(nom)?;

    stockage::sauvegarder(&app, palettes)?;
    Ok(())
}

#[tauri::command]
pub fn ajouter_couleur(app: tauri::AppHandle, id_palette: &str, couleur: Couleur) -> Result<(), String> {
    let mut palettes = charger_palettes(&app)?;

    let palette = palettes.trouver_palette(id_palette).ok_or("Cette palette n'existe pas.")?;
    palette.ajouter_couleur(couleur)?;

    stockage::sauvegarder(&app, palettes)?;
    Ok(())
}

#[tauri::command]
pub fn supprimer_couleur(app: tauri::AppHandle, id_palette: &str, id_couleur: &str) -> Result<(), String> {
    let mut palettes = charger_palettes(&app)?;

    let palette = palettes.trouver_palette(id_palette).ok_or("Cette palette n'existe pas.")?;
    palette.supprimer_couleur(id_couleur)?;

    stockage::sauvegarder(&app, palettes)?;
    Ok(())
}