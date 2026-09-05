<script setup lang="ts">

    import { usePalette } from "../composables/usePalette.ts"
    const { palettes, ajouterPalette } = usePalette();
    import type { Palette } from "../types/palette.ts"
    import type { Couleur } from "../types/couleur.ts"
    import CartePalette from "../components/CartePalette.vue"
    import CarteCouleur from "../components/CarteCouleur.vue"
    import { ref } from "vue"
    const paletteChoisie = ref<Palette | null>(null)
    const couleurChoisie = ref<Couleur | null>(null)
    
    const paletteVide : Palette = {
        id: 0,
        nom: "+",
        couleurs: [],
        createdAt: new Date()
    };
    const nouveauNom = ref("")


</script>



<template>

<div class="container-palettes flex w-full h-full px-2">

    <div class="palettes w-1/4 h-full overflow-y-auto grid gap-2 scrollbar-hide ">

        <CartePalette 
            v-for="palette in palettes"
            :key="palette.id"
            :palette="palette"
            @click="paletteChoisie?.id === palette.id ? paletteChoisie = null : paletteChoisie = palette"
            :selected="paletteChoisie?.id === palette.id"
            :class="{ 'bg-[#e2e8f04D] text-black rounded-l-xl': paletteChoisie?.id === palette.id }"
        />
        <CartePalette
            :key="paletteVide.id"
            :palette="paletteVide"
            :selected="paletteChoisie?.id === paletteVide.id"
            @click="paletteChoisie?.id === paletteVide.id ? paletteChoisie = null : paletteChoisie = paletteVide"
            :class="{ 'bg-[#e2e8f04D] text-black rounded-l-xl': paletteChoisie?.id === paletteVide.id }"
        />

    </div>

    <div class="couleurs w-3/4 h-full overflow-y-auto rounded-r-xl scrollbar-hide"
    :class="{
         'bg-[#e2e8f04D]': paletteChoisie != null
     }"
    >
        <span v-if="!paletteChoisie">Choisir une palette</span>

        <div v-if="paletteChoisie?.id === 0" class="p-3 flex flex-col items-center justify-center text-center h-full">
            <h1 class="text-center text-2xl mb-3">Ajouter une palette</h1>
            
            <h3 class="text-center">Entrez le nom de la nouvelle palette</h3>
            <input 
            type="text" 
            class="border text-white my-3"  
            v-model="nouveauNom"
            @keyup.enter="ajouterPalette(nouveauNom), nouveauNom=''"
            />
            <button
                class="px-4 py-2 rounded-lg 
                        bg-white/40 backdrop-blur-sm 
                        text-black font-semibold
                        hover:bg-white/60 hover:shadow-lg
                        transition-all duration-200"
                @click="ajouterPalette(nouveauNom), nouveauNom=''"
                        
            >
                Ajouter
            </button>
        </div>

        <div 
        v-if="paletteChoisie && paletteChoisie.id !== 0"

        class="grid grid-cols-4 sm:grid-cols-4 gap-1 "
        >
            <CarteCouleur
            
                v-for="couleur in paletteChoisie.couleurs"
                :key="couleur.id"
                class="aspect-square flex items-center justify-center"
                :couleur="couleur"
                @click="couleurChoisie?.id === couleur.id ? couleurChoisie = null : couleurChoisie = couleur"
                :selected="couleurChoisie?.id === couleur.id"
                :class="{ 'bg-[#e2e8f04D] text-black rounded-full': couleurChoisie?.id === couleur.id }"
            />
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