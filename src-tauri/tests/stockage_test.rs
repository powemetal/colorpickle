use color_picker_lib::palettes::palette::Palette;
use color_picker_lib::palettes::palettes::Palettes;
use color_picker_lib::stockage::donnees_exemple;
use std::fs;
use std::path::{Path, PathBuf};

pub fn charger_depuis_chemin(chemin: &Path) -> Result<Vec<Palette>, String> {
    if !chemin.exists() {
        return Ok(donnees_exemple());
    }

    let contenu = fs::read_to_string(chemin)
        .map_err(|e| format!("Erreur lors de la lecture du fichier : {e}"))?;

    let palettes: Vec<Palette> = serde_json::from_str(&contenu)
        .map_err(|e| format!("Erreur lors du décodage du JSON : {e}"))?;

    Ok(palettes)
}

pub fn sauvegarder_vers_chemin(chemin: &Path, palettes: &Palettes) -> Result<(), String> {
    let chemin_temp = chemin.with_extension("json.tmp");

    let json_data = serde_json::to_string_pretty(&palettes.get_palettes())
        .map_err(|e| format!("Erreur de sérialisation JSON : {e}"))?;

    fs::write(&chemin_temp, json_data)
        .map_err(|e| format!("Erreur d'écriture dans le fichier temporaire : {e}"))?;

    fs::rename(chemin_temp, chemin)
        .map_err(|e| format!("Erreur lors du renommage atomique : {e}"))?;

    Ok(())
}

// Helper pour créer un chemin unique dans le dossier temporaire du système
fn generer_chemin_test_temporaire(nom_test: &str) -> PathBuf {
    let mut dossier = std::env::temp_dir();
    dossier.push("color_picker_tests");
    let _ = fs::create_dir_all(&dossier);
    dossier.join(format!("{nom_test}_{}.json", uuid::Uuid::new_v4()))
}

#[test]
fn test_donnees_exemple_valides() {
    // Vérifie que palettes_exemple.json est syntaxiquement valide
    // et désérialisé sans erreur
    let defauts = donnees_exemple();
    assert!(!defauts.is_empty(), "Le fichier de seed ne devrait pas être vide");
    assert!(!defauts[0].id().is_empty());
    assert!(!defauts[0].nom().is_empty());
}

#[test]
fn test_charger_fichier_inexistant_renvoie_donnees_defaut() {
    let chemin_inexistant = generer_chemin_test_temporaire("inexistant");

    // Si le fichier n'existe pas encore, il doit renvoyer les données d'exemple
    let resultat = charger_depuis_chemin(&chemin_inexistant);
    assert!(resultat.is_ok());

    let palettes = resultat.unwrap();
    let defauts = donnees_exemple();
    assert_eq!(palettes.len(), defauts.len());
}

#[test]
fn test_sauvegarder_et_recharger() {
    let chemin = generer_chemin_test_temporaire("sauvegarde");

    // 1. Préparer une palette
    let nouvelle_palette = Palette::new("Palette Sauvegardée".to_string(), vec![]);
    let id_palette = nouvelle_palette.id().to_string();
    let palettes_a_sauvegarder = Palettes::new(vec![nouvelle_palette]);

    // 2. Écriture atomique
    let resultat_save = sauvegarder_vers_chemin(&chemin, &palettes_a_sauvegarder);
    assert!(resultat_save.is_ok());
    assert!(chemin.exists(), "Le fichier JSON doit exister sur le disque");

    // 3. Vérifier que le fichier temporaire n'existe plus
    let chemin_tmp = chemin.with_extension("json.tmp");
    assert!(!chemin_tmp.exists(), "Le fichier .tmp doit avoir été renommé");

    // 4. Relecture depuis le disque
    let resultat_charge = charger_depuis_chemin(&chemin);
    assert!(resultat_charge.is_ok());

    let palettes_lues = resultat_charge.unwrap();
    assert_eq!(palettes_lues.len(), 1);
    assert_eq!(palettes_lues[0].id(), id_palette);
    assert_eq!(palettes_lues[0].nom(), "Palette Sauvegardée");

    // Nettoyage après le test
    let _ = fs::remove_file(chemin);
}

#[test]
fn test_charger_fichier_corrompu_renvoie_erreur() {
    let chemin = generer_chemin_test_temporaire("corrompu");

    // Écrire un JSON invalide dans le fichier
    fs::write(&chemin, "{ ce n'est pas du json valide").unwrap();

    let resultat = charger_depuis_chemin(&chemin);
    assert!(resultat.is_err(), "Un JSON corrompu doit lever une Err");

    // Nettoyage
    let _ = fs::remove_file(chemin);
}