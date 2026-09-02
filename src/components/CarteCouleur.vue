<script setup lang="ts">

import type { Couleur } from "@/types/couleur";

const props = defineProps<{
  couleur: Couleur;
}>();

</script>

<template>
    <figure class="flex items-center justify-center">
        <div 
            class="w-28 h-28 rounded-full flex items-center justify-center border-4 border-white/60 shadow-lg hover:scale-105 hover:shadow-xl transition"
            :style="{ backgroundColor: props.couleur.codeHex }"
        >
            <span v-if="props.couleur.nom" class="text-white font-bold text-center drop-shadow-md px-2">
                {{ props.couleur.nom }}
            </span>
        </div>
    </figure>
</template>