<script setup lang="ts">
    import { palettes, ajouterPalette } from "../composables/usePalettes.ts"
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
            :selected="paletteChoisie?.id === paletteVide.id"@click="paletteChoisie?.id === paletteVide.id ? paletteChoisie = null : paletteChoisie = paletteVide"
            :class="{ 'bg-[#e2e8f04D] text-black rounded-l-xl': paletteChoisie?.id === paletteVide.id }"
        />

    </div>

    <div class="couleurs w-3/4 h-full overflow-y-auto rounded-r-xl scrollbar-hide"
    :class="{
         'bg-[#e2e8f04D]': paletteChoisie != null
     }"
    >
        <span v-if="!paletteChoisie">Choisir une palette</span>

        <div 
        v-if="paletteChoisie"
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