<script setup lang="ts">

    import { usePalette } from "../composables/usePalette.ts"
    const { palettes, ajouterPalette, supprimerPalette, supprimerCouleur, modifierCouleurNom } = usePalette();
    import type { Palette } from "../types/palette.ts"
    import type { Couleur } from "../types/couleur.ts"
    import CartePalette from "../components/CartePalette.vue"
    import CarteCouleur from "../components/CarteCouleur.vue"
    import AjoutPalette from "../components/AjoutPalette.vue"
    import ChoisirPalette from "../components/ChoisirPalette.vue"
    import ChoixCouleurs from "../components/ChoixCouleurs.vue"
    import { ref, computed } from "vue"
    // const paletteChoisie = ref<Palette | null>(null)
    const couleurChoisie = ref<Couleur | null>(null)
    const paletteChoisieId = ref<number | null>(null)
    const nomCouleur = ref("")

    const paletteChoisie = computed(() =>
        palettes.value.find(p => p.id === paletteChoisieId.value) || null
    )


    
    const paletteVide : Palette = {
        id: 0,
        nom: "+",
        couleurs: [],
        createdAt: new Date()
    };
    const nouveauNom = ref("")


</script>



<template>

<div class="flex flex-col h-full">

    
    <div class="flex-1 min-h-0">
        <div class="container-palettes flex w-full h-full px-2">

    
            <div class="palettes w-1/4 h-full overflow-y-auto grid gap-2 scrollbar-hide">
                <CartePalette 
                    v-for="palette in palettes"
                    :key="palette.id"
                    :palette="palette"
                    @click="paletteChoisieId === palette.id ? paletteChoisieId = null : paletteChoisieId = palette.id"
                    :selected="paletteChoisie?.id === palette.id"
                    :class="{ 'bg-[#e2e8f04D] text-black rounded-l-xl': paletteChoisie?.id === palette.id }"
                />

                <CartePalette
                    :key="paletteVide.id"
                    :palette="paletteVide"
                    :selected="paletteChoisieId === paletteVide.id"
                    @click="paletteChoisieId === paletteVide.id ? paletteChoisieId = null : paletteChoisieId = paletteVide.id"
                    :class="{ 'bg-[#e2e8f04D] text-black rounded-l-xl': paletteChoisie?.id === paletteVide.id }"
                />
            </div>

    
            <div class="couleurs w-3/4 h-full overflow-y-auto rounded-r-xl scrollbar-hide"
                :class="{ 'bg-[#e2e8f04D]': paletteChoisie != null }">

                <div v-if="paletteChoisieId === null" class="flex flex-col h-full">
                    <ChoisirPalette/>
                </div>

                <div v-if="paletteChoisieId === 0" class="flex flex-col h-full">
                    <AjoutPalette :ajouterPalette="ajouterPalette" />
                </div>

                <ChoixCouleurs 
                    v-if="paletteChoisie && paletteChoisie.id !== 0"
                    :palette="paletteChoisie"
                    :paletteChoisieId="paletteChoisieId" 
                    @select="couleurChoisie = $event; nomCouleur = $event?.nom || ''"
                />
            </div>

        </div>
    </div>

<div class="flex w-full h-[10%] text-white">

    <div class="mx-5 flex items-center">
        <button
            class="px-2 py-1 
            bg-white/10 
            text-white 
            rounded 
            border border-white/10 
            hover:bg-white/20 
            transition-all duration-150 
            whitespace-nowrap"
            @click="paletteChoisie && supprimerPalette(paletteChoisie); paletteChoisieId=null"
        >
            Supprimer
        </button>
    </div>

    <div class="flex items-center">
        <input 
            type="text" 
            class="border w-40 mr-1"
            v-model="nomCouleur"
        />
    </div>

        <div class="flex items-center  mx-auto">
        <button
        class="px-2 py-1
                bg-white/10
                text-white
                rounded
                border border-white/10
                hover:bg-white/20
                transition-all duration-150
                whitespace-nowrap"
                @click="couleurChoisie && paletteChoisie && modifierCouleurNom(couleurChoisie, paletteChoisie, nomCouleur)"
        >
        Enregistrer
        </button>
        </div>

    <div class="flex items-center ml-auto mr-3">
        <button
        class="px-2 py-1
                bg-white/10
                text-white
                rounded
                border border-white/10
                hover:bg-white/20
                transition-all duration-150
                whitespace-nowrap"
                @click="couleurChoisie && paletteChoisie && supprimerCouleur(couleurChoisie.id, paletteChoisie); couleurChoisie = null"



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