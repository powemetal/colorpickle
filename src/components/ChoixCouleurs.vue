<script setup lang="ts">
    import type { Palette, Couleur } from "../types/palette"
    import { ref } from "vue"
    import ChoisirPalette from "../components/ChoisirPalette.vue"
    import AjoutPalette from "../components/AjoutPalette.vue"
    import CarteCouleur from "../components/CarteCouleur.vue"

    const props = defineProps<{
        palette: Palette | null
    }>()
    const couleurChoisie = ref<Couleur | null>(null)
</script>

<template>
        <!-- palette non choisie -->
        <div v-if="!props.palette" class="flex flex-col h-full"><ChoisirPalette/></div>

        <!-- palette + pour ajouter -->
        <div v-if="props.palette?.id === 0" class="flex flex-col h-full"><AjoutPalette/></div>

        <div 
            v-if="props.palette && props.palette.id !== 0"
            class="grid grid-cols-4 sm:grid-cols-4 gap-1 "
            >
                <CarteCouleur
                
                    v-for="couleur in props.palette.couleurs"
                    :key="couleur.id"
                    class="aspect-square flex items-center justify-center"
                    :couleur="couleur"
                    @click="couleurChoisie?.id === couleur.id ? couleurChoisie = null : couleurChoisie = couleur"
                    :selected="couleurChoisie?.id === couleur.id"
                    :class="{ 'bg-[#e2e8f04D] text-black rounded-full': couleurChoisie?.id === couleur.id }"
                />
        </div>

</template>

