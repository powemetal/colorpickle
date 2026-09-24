use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub enum ErreurCouleur {
    NomVide,
    HexInvalide,
    HexRgbIncoherent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Couleur {
    id: String,
    nom: String,
    valeur_rouge: u8,
    valeur_vert: u8,
    valeur_bleu: u8,
    code_hex: String,
    created_at: NaiveDateTime,
}

// Todo: validation des donnees
impl Couleur {
    pub fn new(
        id: String,
        nom: String,
        valeur_rouge: u8,
        valeur_vert: u8,
        valeur_bleu: u8,
        code_hex: String,
    ) -> Result<Self, ErreurCouleur> {

        if nom.trim().is_empty() {
            return Err(ErreurCouleur::NomVide);
        }
        if !Self::hex_valide(&code_hex) {
            return Err(ErreurCouleur::HexInvalide);
        }

        let hex_rgb = format!("{:02X}{:02X}{:02X}", valeur_rouge, valeur_vert, valeur_bleu);
        if hex_rgb != code_hex.to_uppercase() {
            return Err(ErreurCouleur::HexRgbIncoherent);
        }


        Ok(Self{
            id,
            nom, 
            valeur_rouge, 
            valeur_vert, 
            valeur_bleu, 
            code_hex, 
            created_at: chrono::Utc::now().naive_utc(),
        })
    }

    pub fn id(&self) -> &str { 
        &self.id 
    }
    
    pub fn nom(&self) -> &str { 
        &self.nom 
    }

    pub fn valeur_rouge(&self) -> u8 { 
        self.valeur_rouge 
    }

    pub fn valeur_vert(&self) -> u8 { 
        self.valeur_vert 
    }
    
    pub fn valeur_bleu(&self) -> u8 { 
        self.valeur_bleu 
    }

    pub fn code_hex(&self) -> &str { 
        &self.code_hex 
    }

    pub fn created_at(&self) -> &NaiveDateTime { 
        &self.created_at 
    }

pub fn set_code_hex(&mut self, code_hex: String) -> Result<(), ErreurCouleur> {
    if !Self::hex_valide(&code_hex) {
        return Err(ErreurCouleur::HexInvalide);
    }

    let hex_rgb = format!("{:02X}{:02X}{:02X}", self.valeur_rouge, self.valeur_vert, self.valeur_bleu);
    if hex_rgb != code_hex.to_uppercase() {
        return Err(ErreurCouleur::HexRgbIncoherent);
    }

    self.code_hex = code_hex;
    Ok(())
}

pub fn modifier_couleur_nom(&mut self, nom: String) -> Result<(), ErreurCouleur> {
    if nom.trim().is_empty() {
        return Err(ErreurCouleur::NomVide);
    }
    self.nom = nom;
    Ok(())
}

    fn hex_valide(hex: &str) -> bool {
        hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
    }

}

