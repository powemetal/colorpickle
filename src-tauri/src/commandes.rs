use tauri::AppHandle;

use crate::{
    palettes::{couleur::Couleur, couleur::ErreurCouleur, palette::Palette, palettes::Palettes}, stockage
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

// j'ai essayé de faire passer une classe Couleur mais j'avais des problèmes avec la sérialisation ou déserialisation du created_at
// alors j'ai décidé de manuellement recréer l'objet sans ce champ [MathG]
#[tauri::command]
pub fn ajouter_couleur(
    app: tauri::AppHandle,
    id_palette: String,
    nom: String,
    valeur_rouge: u8,
    valeur_vert: u8,
    valeur_bleu: u8,
    code_hex: String,
    ) -> Result<Couleur, String> {

    let mut palettes = charger_palettes(&app)?;

    let palette = palettes.trouver_palette(&id_palette).ok_or("Cette palette n'existe pas.")?;
    let couleur = Couleur::new(
        uuid::Uuid::new_v4().to_string(), // cette façon de générer un id m'a été proposée par l'IA [MathG]
        nom,
        valeur_rouge,
        valeur_vert,
        valeur_bleu,
        code_hex,
    ).map_err(|e| match e {
        ErreurCouleur::NomVide => "Le nom de la couleur ne peut pas être vide.".to_string(),
        ErreurCouleur::HexInvalide => "Le code hex de la couleur est invalide.".to_string(),
        ErreurCouleur::HexRgbIncoherent => "Le code hex ne correspond pas aux valeurs RGB.".to_string(),
    })?;

    palette.ajouter_couleur(couleur.clone())?;

    stockage::sauvegarder(&app, palettes)?;
    Ok(couleur)
}

#[tauri::command]
pub fn supprimer_couleur(app: tauri::AppHandle, id_palette: &str, id_couleur: &str) -> Result<(), String> {
    let mut palettes = charger_palettes(&app)?;

    let palette = palettes.trouver_palette(id_palette).ok_or("Cette palette n'existe pas.")?;
    palette.supprimer_couleur(id_couleur)?;

    stockage::sauvegarder(&app, palettes)?;
    Ok(())
}

#[tauri::command]
pub fn modifier_couleur_nom(app: tauri::AppHandle, id_palette: &str, id_couleur: &str, nom: String) -> Result<(), String> {
    let mut palettes = charger_palettes(&app)?;
    let palette = palettes.trouver_palette(id_palette).ok_or("Cette palette n'existe pas.")?;

    let couleur_selectionnee = palette.trouver_couleur_mut(&id_couleur).ok_or("Cette couleur n'existe pas.")?;

    couleur_selectionnee.modifier_couleur_nom(nom).map_err(|e| match e {
        ErreurCouleur::NomVide => "Le nom de la couleur ne peut pas être vide.".to_string(),
        ErreurCouleur::HexInvalide => "Le code hex de la couleur est invalide.".to_string(),
        ErreurCouleur::HexRgbIncoherent => "Le code hex ne correspond pas aux valeurs RGB.".to_string(),
    })?;

    stockage::sauvegarder(&app, palettes)?;
    Ok(())
}

// puisqu'on sauvegarde après chaque fonction, le bouton sauvegardé n'avait plus vraiment d'usage il a été changé pour un export de fichier
#[tauri::command]
pub fn exporter_donnees(app: tauri::AppHandle, chemin_destination: String) -> Result<(), String> {
    let palettes = charger_palettes(&app)?;
    let json = serde_json::to_string_pretty(&palettes).map_err(|e| e.to_string())?;

    std::fs::write(chemin_destination, json).map_err(|e| e.to_string())?;

    Ok(())
}
