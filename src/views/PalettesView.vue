<script setup lang="ts">
    import { palettes } from "../data/paletteExamples.ts"
    import type { Palette } from "../types/palette.ts"
    import CartePalette from "../components/CartePalette.vue"
    import { ref } from "vue"
    const paletteChoisie = ref<Palette | null>(null)
</script>



<template>

<div class="container-palettes flex w-full h-full px-2">

    <div class="palettes w-1/4 h-full overflow-y-auto grid gap-2 scrollbar-hide ">

        <CartePalette 
            v-for="palette in palettes"
            :key="palette.id"
            :palette="palette"
            @click="paletteChoisie = palette"
            :selected="paletteChoisie?.id === palette.id"
            :class="{
                'bg-slate-200 text-black rounded-l-xl': paletteChoisie?.id === palette.id
            }"
        />
    </div>

    <div class="couleurs w-3/4 h-full overflow-y-auto bg-slate-200 rounded-r-xl">

    </div>

</div>

</template>

<style>
/* Hide scrollbar but keep scrolling */
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}

.scrollbar-hide {
  -ms-overflow-style: none; /* IE & Edge */
  scrollbar-width: none; /* Firefox */
}
</style>