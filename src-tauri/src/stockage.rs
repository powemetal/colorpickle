use std::{fs, path::PathBuf};

use tauri::{AppHandle, Manager};

use crate::palettes::{palette::Palette, palettes::Palettes};


const NOM_FICHIER: &str = "palettes.json";

pub fn chemin_fichier(app: &AppHandle) -> Result<PathBuf, String> {
    let dossier = app
        .path()
        .app_data_dir()
        .map_err(|erreur| format!("Dossier de données introuvable : {erreur}"))?;

    fs::create_dir_all(&dossier)
        .map_err(|erreur| format!("Impossible de créer le dossier de données : {erreur}"))?;

    Ok(dossier.join(NOM_FICHIER))
}

pub fn charger(app: &AppHandle) -> Result<Vec<Palette>, String> {
    let chemin = chemin_fichier(&app)?;

    if !chemin.exists() {
        return Ok(donnees_exemple());
    }

    let contenu = fs::read_to_string(&chemin)
    .map_err(|e| format!("Erreur lors de la lecture du fichier : {e}"))?;

    let palettes: Vec<Palette> = serde_json::from_str(&contenu)
    .map_err(|e| format!("Erreur lors du décodage du JSON : {e}"))?;

    Ok(palettes)
}

// Fonction pour écrire sécuritairement des données sur le local
// (prévient la corruption de fermeture pendant l'écriture)
pub fn sauvegarder(app: &AppHandle, palettes: Palettes) -> Result<(), String> {
    let chemin = chemin_fichier(app)?;
    let chemin_temp = chemin.with_extension("json.tmp");

    let json_data = serde_json::to_string_pretty(&palettes.get_palettes())
    .map_err(|e| format!("Erreur de sérialisation JSON : {e}"))?;

    // On écrit d'abord dans un fichier temporaire
    fs::write(&chemin_temp, json_data)
    .map_err(|e| format!("Erreur d'écriture dans le fichier temporaire : {e}"))?;

    // On renomme par la suite au fichier permanant si tout s'est bien passé
    fs::rename(chemin_temp, chemin)
    .map_err(|e| format!("Erreur lors du renommage atomique : {}", e))?;

    Ok(())
}


// Fonction pour charger les données de test
pub fn donnees_exemple() -> Vec<Palette> {
    let contenu = include_str!("../data/palettes_exemple.json");
    serde_json::from_str(contenu).unwrap_or_default()
}
