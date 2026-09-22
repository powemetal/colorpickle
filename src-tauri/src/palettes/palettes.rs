// use crate::palettes::Palette;

// pub struct Palettes {
//     palettes: Vec<Palette>,
// }

// impl Palettes {
//     pub fn new() -> Self {
//         Self { palettes: Vec::new() }
//     }

//     pub fn ajouter_palette(&mut self, palette: Palette) {
//         self.palettes.push(palette);
//     }

//     pub fn supprimer_palette(&mut self, id: u32) {
//         self.palettes.retain(|p| p.id() != id);
//     }

//     pub fn trouver_palette(&self, id: u32) -> Option<&Palette> {
//         self.palettes.iter().find(|p| p.id() == id)
//     }

//     pub fn afficher_palettes(&self) -> &[Palette] {
//         &self.palettes
//     }
// }
