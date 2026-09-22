use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(serde::Serialize,Deserialize, Clone)]
pub struct Couleur {
    id: String,
    nom: String,
    valeur_rouge: u8,
    valeur_vert: u8,
    valeur_bleu: u8,
    code_hex: String,
    created_at: NaiveDateTime,
}

impl Couleur {
    pub fn new(
        id: String,
        nom: String,
        valeur_rouge: u8,
        valeur_vert: u8,
        valeur_bleu: u8,
        code_hex: String,
    ) -> Self {
        Self{
            id, 
            nom, 
            valeur_rouge, 
            valeur_vert, 
            valeur_bleu, 
            code_hex, 
            created_at: chrono::Utc::now().naive_utc(),
        }
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

    pub fn set_code_hex(&mut self, code_hex: String) {
        // TODO : Vérification code hex valide
        self.code_hex = code_hex;
    }

    pub fn modifier_couleur_nom(&mut self, nom: String) -> Result<(), String> {
        if nom.trim().is_empty() {
            return Err("Le nom de la couleur ne peut pas être vide".into());
        }
        self.nom = nom;
        Ok(())
    }

}

