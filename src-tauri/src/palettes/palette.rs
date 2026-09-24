use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;

use crate::palettes::couleur::Couleur;

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Palette {
    id: String,
    nom: String,
    couleurs: Vec<Couleur>,
    created_at: NaiveDateTime,
}

impl Palette {
    pub fn new(nom: String, couleurs: Vec<Couleur>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            nom,
            couleurs,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
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

    pub fn set_nom(&mut self, nom: &str) -> Result<(), String> {
        let nb_char_max = 25;

        if nom.trim().is_empty() {
            return Err("Le nom de la palette ne peut pas être vide".into());
        }
        if nom.len() > nb_char_max {
            return Err(format!(
                "Le nom de la palette doit faire au maximum {nb_char_max} charactères."
            ));
        }
        if self.est_nom_duplique(&nom) {
            return Err("Il ne peut pas y avoir deux couleurs avec le même nom.".into());
        }
        self.nom = nom.to_string();
        Ok(())
    }

    fn est_nom_duplique(&self, nom: &str) -> bool {
        self.couleurs
            .iter()
            .any(|c| c.nom().trim().to_ascii_lowercase() == nom.trim().to_ascii_lowercase())
    }

    pub fn ajouter_couleur(&mut self, couleur: Couleur) -> Result<(), String> {
        if self
            .couleurs
            .iter()
            .any(|c| c.code_hex() == couleur.code_hex())
        {
            return Err("Cette couleur est dejà dans la palette".into());
        }
        self.couleurs.push(couleur);
        Ok(())
    }

    pub fn supprimer_couleur(&mut self, id: &str) -> Result<(), String> {
        let avant = self.couleurs.len();
        self.couleurs.retain(|c| c.id() != id);

        if self.couleurs.len() == avant {
            return Err("Couleur introuvable".into());
        }

        Ok(())
    }

    pub fn trouver_couleur_mut(&mut self, id: &str) -> Option<&mut Couleur> {
        self.couleurs.iter_mut().find(|c| c.id() == id)
    }
}