use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Couleur {
    pub id: u32,
    pub nom: String,
    pub valeur_rouge: u8,
    pub valeur_vert: u8,
    pub valeur_bleu: u8,
    pub code_hex: String,
    pub created_at: String, // Chaîne ISO 8601
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Palette {
    pub id: u32,
    pub nom: String,
    pub created_at: String,
    pub couleurs: Vec<Couleur>,
}
