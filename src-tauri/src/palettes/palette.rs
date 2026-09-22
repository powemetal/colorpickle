use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use super::couleur::Couleur;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Palette {
    id: u32,
    nom: String,
    couleurs: Vec<Couleur>,
    created_at: NaiveDateTime,
}

impl Palette {
    pub fn new(
        id: u32,
        nom: String,
        couleurs: Vec<Couleur>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self{
            id,
            nom,
            couleurs,
            created_at
        }
    }

    pub fn id(&self) -> u32 { 
        self.id 
    }
    
    pub fn nom(&self) -> &str { 
        &self.nom 
    }

    pub fn couleurs(&self) -> &Vec<Couleur> { 
        &self.couleurs 
    }

    pub fn created_at(&self) -> &NaiveDateTime { 
        &self.created_at 
    }

    pub fn renommer(&mut self, nom: String) -> Result<(), String> {
        if nom.trim().is_empty() {
            return Err("Le nom de la palette ne peut pas être vide".into());
        }
        self.nom = nom;
        Ok(())
    }

    // pub fn ajouter_couleur(&mut self, couleur: Couleur) -> Result<(), String> {
    //     if self.couleurs.iter().any(|c| c.code_hex == couleur.code_hex()) {
    //         return Err("Cette couleur est dejà dans la palette".into());
    //     }
    //     self.couleurs.push(couleur);
    //     Ok(())
    // }

    // pub fn supprimer_couleur(&mut self, id: u32) -> Result<(), String> {
    //     let avant = self.palettes.len();
    //     self.palettes.retain(|p| p.id() != id);

    //     if self.palettes.len() == avant {
    //         return Err("Palettes introuvable".into())
    //     }

    //     Ok(())
    // }

}

// fonctions nécessitant la liste des palettes donc hors de Palette^
pub fn trouver_palette_mut(palettes: &mut Vec<Palette>, id: u32) -> Result<&mut Palette, String> {
    palettes
        .iter_mut()
        .find(|p| p.id() == id)
        .ok_or_else(|| "Palette introuvable".to_string())
}