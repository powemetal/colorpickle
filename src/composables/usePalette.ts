import { ref } from "vue";
import type { Palette } from "../types/palette.ts";
import { palettes as palettesSource } from "../data/paletteExamples.ts";
import { Couleur } from "../types/couleur.ts";
import { useMessage } from "./useMessage.ts";
import {invoke} from '@tauri-apps/api/core'
import { copierAuPressePapier } from "./useInputMain.ts";

export const palettes = ref<Palette[]>(
  JSON.parse(JSON.stringify(palettesSource)),
);
const { afficherMessage } = useMessage();

async function chargerPalettes() {
  try {
    palettes.value = await invoke<Palette[]>("recuperer_palettes");
    //test pour voir ce que palettes contient
    console.log(palettes.value);
  } catch (error) {
    console.error(error);
    afficherMessage("Erreur lors du chargement des palettes.", true);
  }
}

export function usePalette() {
  async function ajouterCouleur(id_palette: string, donnees: Omit<Couleur, "id" | "createdAt">) {
    try {
      const palette = palettes.value.find((pa) => pa.id === id_palette);
      if (!palette) {
        afficherMessage("Aucune palette n'est sélectionnée.", true);
        return;
      }

      await invoke<Couleur>("ajouter_couleur", {
        idPalette: id_palette,
        ...donnees,
        })

      palettes.value = await invoke<Palette[]>("recuperer_palettes");

      afficherMessage(`Couleur ajoutée à la palette ${palette.nom} !`);

    } catch (error) {
      console.error(error);
      afficherMessage(
        "Erreur lors de l'ajout de la couleur à la palette.",
        true,
      );
    }
  }

  async function supprimerCouleur(id_palette: string, id_couleur: string) {
    try {
      const palette = palettes.value.find((pa) => pa.id === id_palette);
      const couleurExiste = palette?.couleurs.some((c) => c.id === id_couleur);

      if (!palette) {
        afficherMessage("Aucune palette n'est sélectionnée.", true);
        return;
      }

      if (!couleurExiste) {
        afficherMessage("La couleur n'existe pas dans la palette.", true);
        return;
      }

      await invoke<void>("supprimer_couleur", {idPalette: id_palette, idCouleur: id_couleur})
    
      palettes.value = await invoke<Palette[]>("recuperer_palettes");
      afficherMessage("Couleur supprimée !");

    } catch (error) {
      console.error(error);
      afficherMessage("Erreur lors de la suppression de la couleur.", true);
    }
  }

  async function ajouterPalette(nom: string) {
    try {
      const nouvellePalette = await invoke<Palette>("creer_palette", {nom});
      palettes.value = await invoke<Palette[]>("recuperer_palettes");
      console.log(nouvellePalette);
      afficherMessage(`Palette \"${nom}\" ajoutée !`);
    } catch (error) {
      console.error(error);
      afficherMessage("Erreur lors de la création de la palette.", true);
    }
  }

  async function supprimerPalette(id: string) {
    try {
      await invoke<void>("supprimer_palette", {id});
      palettes.value = await invoke<Palette[]>("recuperer_palettes");
      afficherMessage("Palette supprimée !");
    } catch (error) {
      console.error(error);
      afficherMessage("Erreur lors de la supression de la palette.", true);
    }
  }

  async function modifierCouleurNom(id_palette: string, id_couleur: string, nom: string) {
    try {
      const palette = palettes.value.find((pa) => pa.id === id_palette);

      if (!palette) {
        afficherMessage("Aucune palette n'est sélectionnée.", true);
        return;
      }

      const couleur = palette.couleurs.find((couleur) => couleur.id === id_couleur);
      if (!couleur) {
        afficherMessage("Couleur non trouvée.", true);
        return;
      }

      await invoke("modifier_couleur_nom", {idPalette: id_palette, idCouleur: id_couleur, nom})
      palettes.value = await invoke<Palette[]>("recuperer_palettes");

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
      palettes.value = await invoke<Palette[]>("recuperer_palettes");
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
    chargerPalettes,
    supprimerCouleur,
    ajouterPalette,
    supprimerPalette,
    modifierCouleurNom,
    modifierPaletteNom,
  };
}
