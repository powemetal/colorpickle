<script setup lang="ts">
  import { usePalette } from "../composables/usePalette.ts";
  const {
    palettes,
    ajouterPalette,
    supprimerPalette,
    supprimerCouleur,
    modifierCouleurNom,
  } = usePalette();
  import type { Palette } from "../types/palette.ts";
  import type { Couleur } from "../types/couleur.ts";
  import CartePalette from "../components/CartePalette.vue";
  import AjoutPalette from "../components/AjoutPalette.vue";
  import ChoisirPalette from "../components/ChoisirPalette.vue";
  import ChoixCouleurs from "../components/ChoixCouleurs.vue";
  import { ref, computed } from "vue";
  import { useInputMain } from "@/composables/useInputMain.ts";

  const couleurChoisie = ref<Couleur | null>(null);
  const paletteChoisieId = ref<string | null>(null);
  const nomCouleur = ref("");
  const { couleurEstFonce, champRecherche } = useInputMain();

  const paletteChoisie = computed(
    () => palettes.value.find((p) => p.id === paletteChoisieId.value) || null,
  );


const palettesFiltrees = computed(() =>
  palettes.value.filter(p => {
    const recherche = champRecherche.value.toLowerCase();

    const matchNomPalette =
      p.nom.toLowerCase().includes(recherche);

    const matchCouleurs =
      p.couleurs.some(c =>
        c.nom.toLowerCase().includes(recherche)
      );

    return matchNomPalette || matchCouleurs;
  })
);


  const paletteVide: Palette = {
    id: "0",
    nom: "+",
    couleurs: [],
    createdAt: new Date(),
  };
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex-1 min-h-0">
      <div class="container-palettes flex w-full h-full px-2">
        <div
          class="palettes w-1/4 h-full overflow-y-auto grid gap-2 scrollbar-hide"
        >
          <CartePalette
            v-for="palette in palettesFiltrees"
            :key="palette.id"
            :palette="palette"
            @click="
              paletteChoisieId === palette.id
                ? (paletteChoisieId = null)
                : (paletteChoisieId = palette.id)
            "
            :selected="paletteChoisie?.id === palette.id"
            :class="[
              paletteChoisie?.id === palette.id
                ? 'bg-[#e2e8f04D] rounded-l-xl'
                : '',
              couleurEstFonce ? 'text-white' : 'text-black',
            ]"
          />

          <CartePalette
            :key="paletteVide.id"
            :palette="paletteVide"
            :selected="paletteChoisieId === paletteVide.id"
            @click="
              paletteChoisieId === paletteVide.id
                ? (paletteChoisieId = null)
                : (paletteChoisieId = paletteVide.id)
            "
            :class="[
              paletteChoisie?.id === paletteVide.id
                ? 'bg-[#e2e8f04D] rounded-l-xl'
                : '',
              couleurEstFonce ? 'text-white' : 'text-black',
            ]"
          />
        </div>

        <div
          class="couleurs w-3/4 h-full overflow-y-auto rounded-r-xl scrollbar-hide"
          :class="{ 'bg-[#e2e8f04D]': paletteChoisie != null }"
        >
          <!-- Cas 1 : Palette non sélectionnée -->
          <div v-if="paletteChoisieId === null" class="flex flex-col h-full"
          :class="couleurEstFonce ? 'text-white' : 'text-black'">
            <ChoisirPalette />
          </div>

          <!-- Cas 2 : Ajouter une palette -->
          <div v-if="paletteChoisieId === '0'" class="flex flex-col h-full">
            <AjoutPalette :ajouterPalette="ajouterPalette" />
          </div>

          <!-- Cas 3 : Palette valide sélectionnée -->
          <ChoixCouleurs
            v-if="paletteChoisie && paletteChoisie.id !== null"
            :palette="paletteChoisie"
            :paletteChoisieId="paletteChoisieId"
            @select="
              couleurChoisie = $event;
              nomCouleur = $event?.nom || '';
            "
          />
        </div>
      </div>
    </div>

    <div class="flex w-full h-[10%] mt-2"
    :class="couleurEstFonce ? 'text-white' : 'text-black'"
    >
      <div class="mx-5 flex items-center">
        <button
          class="px-2 py-1 bg-white/10 rounded border border-white/10 hover:bg-white/20 transition-all duration-150 whitespace-nowrap"
          :class="couleurEstFonce ? 'text-white' : 'text-black'"
          @click="
            paletteChoisie && supprimerPalette(paletteChoisie.id);
            paletteChoisieId = null;
          "
        >
          Supprimer
        </button>
      </div>

      <div class="flex items-center">
        <input
          type="text"
          class="border w-40 mr-1 px-2 py-1 rounded-md"
          maxlength="15"
          v-model="nomCouleur"
          @keyup.enter="
            couleurChoisie &&
            paletteChoisie &&
            modifierCouleurNom(paletteChoisie.id, couleurChoisie.id, nomCouleur)
          "
        />
      </div>

      <div class="flex items-center mx-auto">
        <button
          class="px-2 py-1 bg-white/10  rounded border border-white/10 hover:bg-white/20 transition-all duration-150 whitespace-nowrap"
          :class="couleurEstFonce ? 'text-white' : 'text-black'"
          @click="
            couleurChoisie &&
            paletteChoisie &&
            modifierCouleurNom(paletteChoisie.id, couleurChoisie.id, nomCouleur)
          "
        >
          Enregistrer
        </button>
      </div>

      <div class="flex items-center ml-auto mr-3">
        <button
          class="px-2 py-1 bg-white/10  rounded border border-white/10 hover:bg-white/20 transition-all duration-150 whitespace-nowrap"
          :class="couleurEstFonce ? 'text-white' : 'text-black'"
          @click="
            couleurChoisie &&
              paletteChoisie &&
              supprimerCouleur(paletteChoisie.id, couleurChoisie.id);
            ((couleurChoisie = null), (nomCouleur = ''));
          "
        >
          Supprimer
        </button>
      </div>
    </div>
  </div>
</template>

<style>
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}

.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
