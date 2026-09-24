<script setup lang="ts">
    import type { Palette } from "../types/palette"
    import { ref } from "vue"
    import CarteCouleur from "../components/CarteCouleur.vue"
import { Couleur } from "@/types/couleur.ts"
import { copierAuPressePapier } from "@/composables/useInputMain.ts"

    const props = defineProps<{
        palette: Palette | null
        paletteChoisieId: string | null
    }>()
    const couleurChoisie = ref<Couleur | null>(null)
    
    defineEmits(['select'])

    

</script>

<template>
        <div 
            v-if="props.palette && props.palette.id !== null"
            class="grid grid-cols-4 sm:grid-cols-4 gap-1 "
            >
                <CarteCouleur
                
                    v-for="couleur in props.palette.couleurs"
                    :key="couleur.id"
                    class="aspect-square flex items-center justify-center"
                    :couleur="couleur"
                    @click=" if (couleurChoisie?.id === couleur.id) {
                      couleurChoisie = null ;  
                      $emit('select', null);
                    } else {
                        couleurChoisie = couleur;
                        $emit('select', couleur);
                        copierAuPressePapier(`${couleur.codeHex}`);
                    }"
                    :selected="couleurChoisie?.id === couleur.id"
                    :class="{ 'bg-[#e2e8f04D] text-black rounded-full': couleurChoisie?.id === couleur.id }"
                />
        </div>

</template>

