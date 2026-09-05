<script setup lang="ts">
import Slider from "../components/Slider.vue";
import { useInputMain } from "../composables/useInputMain.ts";
import type { Palette } from "../types/palette.ts";
import { usePalette } from "../composables/usePalette.ts";
import { ref, computed } from "vue";
import { Couleur } from "@/types/couleur.ts";
const { palettes, ajouterCouleur } = usePalette();
const paletteChoisieId = ref<number | null>(palettes.value[0].id);
const paletteChoisie = computed(
  () => palettes.value.find((p) => p.id === paletteChoisieId.value) || null,
);

const {
  displayValue,
  r,
  g,
  b,
  couleurEstFonce,
  onSliderChange,
  onInput,
  onChange,
  copierAuPressePapier,
} = useInputMain();



function couleurActuelle(): Couleur {
  return {
    id: Date.now(),
    nom: "",
    valeurRouge: r.value,
    valeurVert: g.value,
    valeurBleu: b.value,
    codeHex: "#" + displayValue.value,
    createdAt: new Date(),
  };
}
</script>

<template>
  <div class="container mx-auto flex flex-1 flex-col">
    <div class="flex mt-auto m-4 gap-4">
      <div class="flex flex-1 flex-col gap-2 items-center">
        <Slider v-model="r" @input="onSliderChange" couleur="rouge" />
        <Slider v-model="g" @input="onSliderChange" couleur="vert" />
        <Slider v-model="b" @input="onSliderChange" couleur="bleu" />
      </div>
      <div
        class="flex flex-col justify-center w-40"
        :class="couleurEstFonce ? 'text-white' : 'text-black'"
      >
        <div
          class="flex w-full justify-center font-bold bg-gray-700/20 hover:bg-gray-600/40 transition rounded-full px-3 py-1"
        >
          <span class="text-lg">#</span>
          <input
            class="w-20 bg-transparent outline-none border-none text-lg [text-shadow:_inherit]"
            maxlength="6"
            v-model.trim="displayValue"
            @change="onChange"
            @input="onInput"
            @dblclick="copierAuPressePapier(`#${displayValue}`)"
          />
        </div>
        <div class="flex items-center mt-4 mx-auto w-full min-w-0 gap-2">
          <select
            v-model.number="paletteChoisieId"
            class="flex-1 min-w-0 text-center bg-gray-800/20 hover:bg-gray-600/40 transition rounded-full px-3 py-1 truncate border border-gray-600 font-semibold"
            :class="couleurEstFonce ? 'text-white' : 'text-black'"
          >
            <option
              class="text-black text-start"
              v-for="p in palettes"
              :key="p.id"
              :value="p.id"
            >
              {{ p.nom }}
            </option>
          </select>

          <button
            class="h-8 w-8 bg-gray-700/20 hover:bg-gray-600/40 transition rounded-full px-2 flex items-center hover:cursor-pointer disabled:cursor-not-allowed"
            @click="
              paletteChoisie &&
                ajouterCouleur(couleurActuelle(), paletteChoisie);
              console.log(couleurActuelle());
            "
            :disabled="!paletteChoisie"
          >
            <i class="fa-solid fa-plus"></i>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
