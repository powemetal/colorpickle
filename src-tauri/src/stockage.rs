use std::{fmt::format, fs, path::PathBuf};

use tauri::{AppHandle, Manager};

//use crate::modeles::{Couleur, Palette};
use crate::palettes::{couleur::Couleur, palette::Palette};

const NOM_FICHIER: &str = "palettes.json";

fn chemin_fichier(app: &AppHandle) -> Result<PathBuf, String> {
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
pub fn sauvegarder(app: &AppHandle, palettes: Vec<Palette>) -> Result<(), String> {
    let chemin = chemin_fichier(app)?;
    let chemin_temp = chemin.with_extension("json.tmp");

    let json_data = serde_json::to_string_pretty(&palettes)
    .map_err(|e| format!("Erreur de sérialisation JSON : {e}"))?;

    // On écrit d'abord dans un fichier temporaire
    fs::write(&chemin_temp, json_data)
    .map_err(|e| format!("Erreur d'écriture dans le fichier temporaire : {e}"))?;

    // On renomme par la suite au fichier permanant si tout s'est bien passé
    fs::rename(chemin_temp, chemin)
    .map_err(|e| format!("Erreur lors du renommage atomique : {}", e))?;

    Ok(())
}


// // Fonction pour charger les données de test
// fn donnees_exemple() -> Vec<Palette> {
//     let now = "2026-09-19T00:00:00.000Z";

//     vec![
//         Palette {
//             id: 1,
//             nom: "Chaude".to_string(),
//             created_at: now.to_string(),
//             couleurs: vec![
//                 Couleur { id: 1, nom: "Rouge vif".to_string(), valeur_rouge: 255, valeur_vert: 0, valeur_bleu: 0, code_hex: "#FF0000".to_string(), created_at: now.to_string() },
//                 Couleur { id: 2, nom: "Orange".to_string(), valeur_rouge: 255, valeur_vert: 165, valeur_bleu: 0, code_hex: "#FFA500".to_string(), created_at: now.to_string() },
//                 Couleur { id: 3, nom: "Jaune".to_string(), valeur_rouge: 255, valeur_vert: 255, valeur_bleu: 0, code_hex: "#FFFF00".to_string(), created_at: now.to_string() },
//             ],
//         },
//         Palette {
//             id: 2,
//             nom: "Froide".to_string(),
//             created_at: now.to_string(),
//             couleurs: vec![
//                 Couleur { id: 4, nom: "Bleu océan".to_string(), valeur_rouge: 0, valeur_vert: 105, valeur_bleu: 148, code_hex: "#006994".to_string(), created_at: now.to_string() },
//                 Couleur { id: 5, nom: "Bleu ciel".to_string(), valeur_rouge: 135, valeur_vert: 206, valeur_bleu: 235, code_hex: "#87CEEB".to_string(), created_at: now.to_string() },
//                 Couleur { id: 6, nom: "Turquoise".to_string(), valeur_rouge: 64, valeur_vert: 224, valeur_bleu: 208, code_hex: "#40E0D0".to_string(), created_at: now.to_string() },
//             ],
//         },
//         Palette {
//             id: 3,
//             nom: "Neutre".to_string(),
//             created_at: now.to_string(),
//             couleurs: vec![
//                 Couleur { id: 7, nom: "Noir".to_string(), valeur_rouge: 0, valeur_vert: 0, valeur_bleu: 0, code_hex: "#000000".to_string(), created_at: now.to_string() },
//                 Couleur { id: 8, nom: "Gris".to_string(), valeur_rouge: 128, valeur_vert: 128, valeur_bleu: 128, code_hex: "#808080".to_string(), created_at: now.to_string() },
//                 Couleur { id: 9, nom: "Blanc".to_string(), valeur_rouge: 255, valeur_vert: 255, valeur_bleu: 255, code_hex: "#FFFFFF".to_string(), created_at: now.to_string() },
//             ],
//         },
//         Palette {
//             id: 4,
//             nom: "Pastel".to_string(),
//             created_at: now.to_string(),
//             couleurs: vec![
//                 Couleur { id: 10, nom: "Rose doux".to_string(), valeur_rouge: 255, valeur_vert: 182, valeur_bleu: 193, code_hex: "#FFB6C1".to_string(), created_at: now.to_string() },
//                 Couleur { id: 11, nom: "Pêche".to_string(), valeur_rouge: 255, valeur_vert: 218, valeur_bleu: 185, code_hex: "#FFDAB9".to_string(), created_at: now.to_string() },
//                 Couleur { id: 12, nom: "Lavande".to_string(), valeur_rouge: 230, valeur_vert: 230, valeur_bleu: 250, code_hex: "#E6E6FA".to_string(), created_at: now.to_string() },
//                 Couleur { id: 13, nom: "Menthe".to_string(), valeur_rouge: 152, valeur_vert: 255, valeur_bleu: 152, code_hex: "#98FF98".to_string(), created_at: now.to_string() },
//                 Couleur { id: 14, nom: "Bleu pâle".to_string(), valeur_rouge: 173, valeur_vert: 216, valeur_bleu: 230, code_hex: "#ADD8E6".to_string(), created_at: now.to_string() },
//                 Couleur { id: 15, nom: "Jaune pâle".to_string(), valeur_rouge: 255, valeur_vert: 255, valeur_bleu: 224, code_hex: "#FFFFE0".to_string(), created_at: now.to_string() },
//                 Couleur { id: 16, nom: "Vert clair".to_string(), valeur_rouge: 144, valeur_vert: 238, valeur_bleu: 144, code_hex: "#90EE90".to_string(), created_at: now.to_string() },
//                 Couleur { id: 17, nom: "Bleu bébé".to_string(), valeur_rouge: 137, valeur_vert: 207, valeur_bleu: 240, code_hex: "#89CFF0".to_string(), created_at: now.to_string() },
//                 Couleur { id: 18, nom: "Corail".to_string(), valeur_rouge: 255, valeur_vert: 127, valeur_bleu: 80, code_hex: "#FF7F50".to_string(), created_at: now.to_string() },
//                 Couleur { id: 19, nom: "Mauve".to_string(), valeur_rouge: 224, valeur_vert: 176, valeur_bleu: 255, code_hex: "#E0B0FF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 20, nom: "Bleu gris".to_string(), valeur_rouge: 176, valeur_vert: 196, valeur_bleu: 222, code_hex: "#B0C4DE".to_string(), created_at: now.to_string() },
//                 Couleur { id: 21, nom: "Saumon".to_string(), valeur_rouge: 250, valeur_vert: 128, valeur_bleu: 114, code_hex: "#FA8072".to_string(), created_at: now.to_string() },
//                 Couleur { id: 22, nom: "Crème".to_string(), valeur_rouge: 255, valeur_vert: 253, valeur_bleu: 208, code_hex: "#FFFDD0".to_string(), created_at: now.to_string() },
//                 Couleur { id: 23, nom: "Bleu doux".to_string(), valeur_rouge: 191, valeur_vert: 239, valeur_bleu: 255, code_hex: "#BFEFFF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 24, nom: "Vert eau".to_string(), valeur_rouge: 193, valeur_vert: 255, valeur_bleu: 193, code_hex: "#C1FFC1".to_string(), created_at: now.to_string() },
//                 Couleur { id: 25, nom: "Lilas".to_string(), valeur_rouge: 200, valeur_vert: 162, valeur_bleu: 200, code_hex: "#C8A2C8".to_string(), created_at: now.to_string() },
//                 Couleur { id: 26, nom: "Beige".to_string(), valeur_rouge: 245, valeur_vert: 245, valeur_bleu: 220, code_hex: "#F5F5DC".to_string(), created_at: now.to_string() },
//                 Couleur { id: 27, nom: "Rose pâle".to_string(), valeur_rouge: 255, valeur_vert: 192, valeur_bleu: 203, code_hex: "#FFC0CB".to_string(), created_at: now.to_string() },
//                 Couleur { id: 28, nom: "Bleu perle".to_string(), valeur_rouge: 205, valeur_vert: 215, valeur_bleu: 230, code_hex: "#CDD7E6".to_string(), created_at: now.to_string() },
//             ],
//         },
//         Palette {
//             id: 5,
//             nom: "Vibrante".to_string(),
//             created_at: now.to_string(),
//             couleurs: vec![
//                 Couleur { id: 29, nom: "Rouge feu".to_string(), valeur_rouge: 255, valeur_vert: 69, valeur_bleu: 0, code_hex: "#FF4500".to_string(), created_at: now.to_string() },
//                 Couleur { id: 30, nom: "Magenta".to_string(), valeur_rouge: 255, valeur_vert: 0, valeur_bleu: 255, code_hex: "#FF00FF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 31, nom: "Cyan".to_string(), valeur_rouge: 0, valeur_vert: 255, valeur_bleu: 255, code_hex: "#00FFFF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 32, nom: "Vert lime".to_string(), valeur_rouge: 0, valeur_vert: 255, valeur_bleu: 0, code_hex: "#00FF00".to_string(), created_at: now.to_string() },
//                 Couleur { id: 33, nom: "Bleu roi".to_string(), valeur_rouge: 65, valeur_vert: 105, valeur_bleu: 225, code_hex: "#4169E1".to_string(), created_at: now.to_string() },
//                 Couleur { id: 34, nom: "Rose néon".to_string(), valeur_rouge: 255, valeur_vert: 20, valeur_bleu: 147, code_hex: "#FF1493".to_string(), created_at: now.to_string() },
//                 Couleur { id: 35, nom: "Jaune vif".to_string(), valeur_rouge: 255, valeur_vert: 255, valeur_bleu: 0, code_hex: "#FFFF00".to_string(), created_at: now.to_string() },
//                 Couleur { id: 36, nom: "Vert pomme".to_string(), valeur_rouge: 141, valeur_vert: 182, valeur_bleu: 0, code_hex: "#8DB600".to_string(), created_at: now.to_string() },
//                 Couleur { id: 37, nom: "Bleu nuit".to_string(), valeur_rouge: 25, valeur_vert: 25, valeur_bleu: 112, code_hex: "#191970".to_string(), created_at: now.to_string() },
//                 Couleur { id: 38, nom: "Rouge cerise".to_string(), valeur_rouge: 222, valeur_vert: 49, valeur_bleu: 99, code_hex: "#DE3163".to_string(), created_at: now.to_string() },
//                 Couleur { id: 39, nom: "Orange vif".to_string(), valeur_rouge: 255, valeur_vert: 140, valeur_bleu: 0, code_hex: "#FF8C00".to_string(), created_at: now.to_string() },
//                 Couleur { id: 40, nom: "Bleu flash".to_string(), valeur_rouge: 0, valeur_vert: 191, valeur_bleu: 255, code_hex: "#00BFFF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 41, nom: "Vert néon".to_string(), valeur_rouge: 57, valeur_vert: 255, valeur_bleu: 20, code_hex: "#39FF14".to_string(), created_at: now.to_string() },
//                 Couleur { id: 42, nom: "Rose vif".to_string(), valeur_rouge: 255, valeur_vert: 105, valeur_bleu: 180, code_hex: "#FF69B4".to_string(), created_at: now.to_string() },
//                 Couleur { id: 43, nom: "Bleu pur".to_string(), valeur_rouge: 0, valeur_vert: 0, valeur_bleu: 255, code_hex: "#0000FF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 44, nom: "Vert forêt".to_string(), valeur_rouge: 34, valeur_vert: 139, valeur_bleu: 34, code_hex: "#228B22".to_string(), created_at: now.to_string() },
//                 Couleur { id: 45, nom: "Rouge sang".to_string(), valeur_rouge: 138, valeur_vert: 3, valeur_bleu: 3, code_hex: "#8A0303".to_string(), created_at: now.to_string() },
//                 Couleur { id: 46, nom: "Bleu glace".to_string(), valeur_rouge: 153, valeur_vert: 204, valeur_bleu: 255, code_hex: "#99CCFF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 47, nom: "Jaune or".to_string(), valeur_rouge: 255, valeur_vert: 215, valeur_bleu: 0, code_hex: "#FFD700".to_string(), created_at: now.to_string() },
//                 Couleur { id: 48, nom: "Violet".to_string(), valeur_rouge: 148, valeur_vert: 0, valeur_bleu: 211, code_hex: "#9400D3".to_string(), created_at: now.to_string() },
//                 Couleur { id: 49, nom: "Rouge néon".to_string(), valeur_rouge: 255, valeur_vert: 0, valeur_bleu: 102, code_hex: "#FF0066".to_string(), created_at: now.to_string() },
//                 Couleur { id: 50, nom: "Bleu laser".to_string(), valeur_rouge: 0, valeur_vert: 102, valeur_bleu: 255, code_hex: "#0066FF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 51, nom: "Vert flash".to_string(), valeur_rouge: 0, valeur_vert: 255, valeur_bleu: 128, code_hex: "#00FF80".to_string(), created_at: now.to_string() },
//                 Couleur { id: 52, nom: "Rose shock".to_string(), valeur_rouge: 255, valeur_vert: 0, valeur_bleu: 204, code_hex: "#FF00CC".to_string(), created_at: now.to_string() },
//                 Couleur { id: 53, nom: "Jaune acide".to_string(), valeur_rouge: 204, valeur_vert: 255, valeur_bleu: 0, code_hex: "#CCFF00".to_string(), created_at: now.to_string() },
//                 Couleur { id: 54, nom: "Bleu turbo".to_string(), valeur_rouge: 0, valeur_vert: 153, valeur_bleu: 255, code_hex: "#0099FF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 55, nom: "Orange pop".to_string(), valeur_rouge: 255, valeur_vert: 102, valeur_bleu: 0, code_hex: "#FF6600".to_string(), created_at: now.to_string() },
//                 Couleur { id: 56, nom: "Violet pop".to_string(), valeur_rouge: 170, valeur_vert: 0, valeur_bleu: 255, code_hex: "#AA00FF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 57, nom: "Bleu néon".to_string(), valeur_rouge: 0, valeur_vert: 255, valeur_bleu: 204, code_hex: "#00FFCC".to_string(), created_at: now.to_string() },
//                 Couleur { id: 58, nom: "Rouge flash".to_string(), valeur_rouge: 255, valeur_vert: 51, valeur_bleu: 51, code_hex: "#FF3333".to_string(), created_at: now.to_string() },
//                 Couleur { id: 59, nom: "Vert acide".to_string(), valeur_rouge: 102, valeur_vert: 255, valeur_bleu: 0, code_hex: "#66FF00".to_string(), created_at: now.to_string() },
//                 Couleur { id: 60, nom: "Bleu vif".to_string(), valeur_rouge: 51, valeur_vert: 153, valeur_bleu: 255, code_hex: "#3399FF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 61, nom: "Rose flash".to_string(), valeur_rouge: 255, valeur_vert: 51, valeur_bleu: 153, code_hex: "#FF3399".to_string(), created_at: now.to_string() },
//                 Couleur { id: 62, nom: "Jaune néon".to_string(), valeur_rouge: 255, valeur_vert: 255, valeur_bleu: 102, code_hex: "#FFFF66".to_string(), created_at: now.to_string() },
//                 Couleur { id: 63, nom: "Bleu cobalt".to_string(), valeur_rouge: 0, valeur_vert: 71, valeur_bleu: 171, code_hex: "#0047AB".to_string(), created_at: now.to_string() },
//                 Couleur { id: 64, nom: "Vert laser".to_string(), valeur_rouge: 0, valeur_vert: 255, valeur_bleu: 57, code_hex: "#00FF39".to_string(), created_at: now.to_string() },
//                 Couleur { id: 65, nom: "Rouge vif+".to_string(), valeur_rouge: 255, valeur_vert: 0, valeur_bleu: 51, code_hex: "#FF0033".to_string(), created_at: now.to_string() },
//                 Couleur { id: 66, nom: "Bleu arctiq".to_string(), valeur_rouge: 102, valeur_vert: 204, valeur_bleu: 255, code_hex: "#66CCFF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 67, nom: "Violet néon".to_string(), valeur_rouge: 204, valeur_vert: 0, valeur_bleu: 255, code_hex: "#CC00FF".to_string(), created_at: now.to_string() },
//                 Couleur { id: 68, nom: "Orange pop+".to_string(), valeur_rouge: 255, valeur_vert: 102, valeur_bleu: 51, code_hex: "#FF6633".to_string(), created_at: now.to_string() },
//             ],
//         },
//     ]
// }


fn donnees_exemple() -> Vec<Palette> {
    let now = chrono::NaiveDate::from_ymd_opt(2026, 9, 19)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    vec![Palette::new(1, "Chaude".to_string(), vec![
        Couleur::new(1, "Rouge vif".to_string(), 255, 0, 0, "#FF0000".to_string(), now),
    ], now)]
}