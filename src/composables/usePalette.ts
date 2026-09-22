import { ref } from "vue";
import type { Palette } from "../types/palette.ts";
import { palettes as palettesSource } from "../data/paletteExamples.ts";
import { Couleur } from "../types/couleur.ts";
import { useMessage } from "./useMessage.ts";
import {invoke} from '@tauri-apps/api/core'

export const palettes = ref<Palette[]>(
  JSON.parse(JSON.stringify(palettesSource)),
);
const { afficherMessage } = useMessage();

export function usePalette() {
  function ajouterCouleur(c: Couleur, p: Palette) {
    try {
      const palette = palettes.value.find((pa) => pa.id === p.id);
      if (!palette) {
        afficherMessage("Aucune palette n'est sélectionnée.", true);
        return;
      }
      palette.couleurs.push(c);
      afficherMessage(`Couleur ajoutée à la palette ${palette.nom} !`);
    } catch (error) {
      console.error(error);
      afficherMessage(
        "Erreur lors de l'ajout de la couleur à la palette.",
        true,
      );
    }
  }

  function supprimerCouleur(id: number, p: Palette) {
    try {
      const palette = palettes.value.find((pa) => pa.id === p.id);
      if (!palette) {
        afficherMessage("Aucune palette n'est sélectionnée.", true);
        return;
      }
      palette.couleurs = palette.couleurs.filter((c) => c.id !== id);
      afficherMessage("Couleur supprimée !");
    } catch (error) {
      console.error(error);
      afficherMessage("Erreur lors de la suppression d'une couleur.", true);
    }
  }

  function ajouterPalette(nom: string) {
    try {
      palettes.value.push({
        id: Date.now(),
        nom,
        createdAt: new Date(),
        couleurs: [],
      });
      afficherMessage(`Palette \"${nom}\" ajoutée !`);
    } catch (error) {
      console.error(error);
      afficherMessage("Erreur lors de la création de la palette.", true);
    }
  }

  function supprimerPalette(p: Palette) {
    try {
      palettes.value = palettes.value.filter((palette) => palette.id !== p.id);
      afficherMessage("Palette supprimée !");
    } catch (error) {
      console.error(error);
      afficherMessage("Erreur lors de la supression de la palette.", true);
    }
  }

  function modifierCouleurNom(c: Couleur, p: Palette, nom: string) {
    try {
      const couleur = p.couleurs.find((coul) => coul.id === c.id);
      if (!couleur) {
        afficherMessage("Couleur non trouvée.", true);
        return;
      }
      couleur.nom = nom;
      afficherMessage("Nom de la couleur modifiée !");
    } catch (error) {
      console.error(error);
      afficherMessage(
        "Erreur lors de la modification du nom de la couleur.",
        true,
      );
    }
  }

  async function modifierPaletteNom(p: Palette, nom: string) {
    try {
      await invoke("modifier_palette_nom", {id: p.id, nom});
      p.nom = nom;
      afficherMessage("Nom de la palette modifiée !");
    } catch (error) {
      console.error(error);
      afficherMessage(
        "Erreur lors de la modification du nom de la palette.",
        true,
      );
    }
  }

  return {
    palettes,
    ajouterCouleur,
    supprimerCouleur,
    ajouterPalette,
    supprimerPalette,
    modifierCouleurNom,
    modifierPaletteNom,
  };
}
