use serde::Deserialize;
use chrono::NaiveDateTime;

pub struct Couleur {
    id: u32,
    nom: String,
    valeur_rouge: u8,
    valeur_vert: u8,
    valeur_bleu: u8,
    code_hex: String,
    created_at: NaiveDateTime,
}

impl Couleur {
    pub fn new(
        id: u32,
        nom: String,
        valeur_rouge: u8,
        valeur_vert: u8,
        valeur_bleu: u8,
        code_hex: String,
        created_at: NaiveDateTime,
    ) -> Self {
        Self{
            id, 
            nom, 
            valeur_rouge, 
            valeur_vert, 
            valeur_bleu, 
            code_hex, 
            created_at
        }
    }

    pub fn id(&self) -> u32 { 
        self.id 
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

}

