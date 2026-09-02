<script setup lang="ts">
import type { Palette } from "@/types/palette";

const props = defineProps<{
  palette: Palette;
}>();
</script>

<template>
  <figure class="flex flex-col items-center justify-center">
    <div class="w-28 h-28 rounded-full flex items-center justify-center border-4 border-white/60 shadow-lg hover:scale-105 hover:shadow-xl transition" :style="{ backgroundColor: props.palette.couleurs[0].codeHex }"></div>
    <span v-if="props.palette.nom" class="text-white font-bold text-center drop-shadow-md px-2">
      {{ props.palette.nom }}
    </span>
</figure>
</template>
