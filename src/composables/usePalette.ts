import { ref } from "vue";
import type { Palette } from "../types/palette.ts";
import { palettes as palettesSource } from "../data/paletteExamples.ts";
import { Couleur } from "../types/couleur.ts";

export const palettes = ref<Palette[]>(JSON.parse(JSON.stringify(palettesSource)));


export function usePalette() {


    function ajouterCouleur(c: Couleur, p: Palette) {
        const palette = palettes.value.find(pa => pa.id === p.id);
        if (!palette) return;

        palette.couleurs.push(c)
    }

    function supprimerCouleur(id: number, p: Palette) {
        const palette = palettes.value.find(pa => pa.id === p.id);
        if (!palette) return;
        palette.couleurs = palette.couleurs.filter(c => c.id !== id);
    }

    function ajouterPalette(nom: string) {
        palettes.value.push({
            id: Date.now(),
            nom,
            createdAt: new Date(),
            couleurs: []
        })
    }

    function supprimerPalette(p: Palette) {
        palettes.value = palettes.value.filter(palette => palette.id !== p.id)
    }

    function modifierCouleurNom(c: Couleur, p: Palette, nom: string) {
        const couleur = p.couleurs.find(coul => coul.id === c.id);
        if (!couleur) return;
        couleur.nom = nom
    }

    function modifierPaletteNom(p: Palette, nom : string) {
        p.nom = nom
    }

    return {
        palettes,
        ajouterCouleur,
        supprimerCouleur,
        ajouterPalette,
        supprimerPalette,
        modifierCouleurNom,
        modifierPaletteNom
    };
}


