use crate::palettes::palette::Palette;



// pub struct Palettes {
//     palettes: Vec<Palette>,
// }

impl Palettes {
    pub fn new(palettes: Vec<Palette>) -> Self {
        Self { palettes }
    }

//     pub fn ajouter_palette(&mut self, palette: Palette) {
//         self.palettes.push(palette);
//     }

    pub fn supprimer_palette(&mut self, id: &str) -> Result<(), String> {
        let avant = self.palettes.len();
        self.palettes.retain(|p| p.id() != id);

         if self.palettes.len() == avant {
            return Err("Palette introuvable".into());
        }
        Ok(())
    }

    pub fn trouver_palette(&mut self, id: &str) -> Option<&mut Palette> {
        self.palettes.iter_mut().find(|p| p.id() == id)
    }

    pub fn get_palettes(&self) -> &[Palette] {
        &self.palettes
    }
}
