<script setup lang="ts">

import type { Couleur } from "@/types/couleur";
import { useCouleur } from "@/composables/useCouleur"

const { estFonce } = useCouleur();


const props = defineProps<{
  couleur: Couleur;
  selected?: boolean
}>();

</script>

<template>
    <figure class="flex items-center justify-center">
        <div 
            class="w-17 h-17 rounded-full flex items-center justify-center hover:scale-105 hover:shadow-xl transition shrink-0"
            :class="selected ? 'border-4 border-white/60 shadow-lg' : ''"
            :style="{ backgroundColor: props.couleur.codeHex }"
        >
            <span 
                v-if="props.couleur.nom" 
                lang="fr"
                class="font-bold text-center text-xs drop-shadow-md px-2 break-words leading-tight line-clamp-2"
                :class="estFonce(
                    props.couleur.valeurRouge,
                    props.couleur.valeurVert,
                    props.couleur.valeurBleu,
                ) ? 'text-white' : 'text-black'"
            >
                {{ props.couleur.nom }}
            </span>
        </div>
    </figure>
</template>