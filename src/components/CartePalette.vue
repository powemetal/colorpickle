<script setup lang="ts">
import type { Palette } from "@/types/palette";


const props = defineProps<{
  palette: Palette;
  selected?: boolean
}>();
</script>

<template>
  <figure class="flex flex-col items-center justify-center">
    <div 
    class="w-18 h-18 rounded-full flex items-center justify-center  hover:scale-105 hover:shadow-xl transition mt-1" 
    :class="selected ? 'border-4 border-white/60 shadow-lg' : ''"
    :style="{ backgroundColor: props.palette.couleurs[0]?.codeHex || '#ccc' }"
    >
  </div>
    
    <span v-if="props.palette.nom" :class="selected ? 'text-black font-bold' : 'text-white-50 font-normal'" class="text-center">
      {{ props.palette.nom }}
    </span>
</figure>
</template>