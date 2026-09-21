use serde::Deserialize;
use chrono::NaiveDateTime;
use palettes::Couleur;




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

    pub fn modifier_palette_nom(&mut self, nom: String) -> Result<(), String> {
        if nom.trim().is_empty() {
            return Err("Le nom de la palette ne peut pas être vide".into());
        }
        self.nom = nom;
        Ok(())
    }

    pub fn ajouter_couleur(&mut self, couleur: Couleur) -> Result<(), String> {
        if self.couleurs.iter().any(|c| c.code_hex == couleur.code_hex()) {
            return Err("Cette couleur est dejà dans la palette".into());
        }
        self.couleurs.push(couleur);
        Ok(())
    }

    pub fn supprimer_couleur(&mut self, id: u32) -> Result<(), String> {
        let avant = self.palettes.len();
        self.palettes.retain(|p| p.id() != id);

        if self.palettes.len() == avant {
            return Err("Palettes introuvable".into())
        }

        Ok(())
    }



}