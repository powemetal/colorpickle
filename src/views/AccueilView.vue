<script setup lang="ts">
import Slider from "../components/Slider.vue";
import { useInputMain } from "../composables/useInputMain.ts";

const {
  displayValue,
  r,
  g,
  b,
  paletteCourante,
  onSliderChange,
  onInput,
  onChange,
  copierAuPressePapier,
  estFonce,
} = useInputMain();
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
        :class="estFonce(r, g, b) ? 'text-white' : 'text-black'"
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
          <span
            class="flex-1 min-w-0 text-center bg-gray-800/20 transition rounded-full px-3 py-1 truncate border border-gray-600 font-semibold"
            >{{ paletteCourante.nom }}</span
          >
          <!-- TODO: ajouter la fonction pour ajouter la couleur à la palette -->
          <button
            class="h-8 w-8 bg-gray-700/20 hover:bg-gray-600/40 transition rounded-full px-2 flex items-center hover:cursor-pointer"
            @click=""
          >
            <i class="fa-solid fa-plus"></i>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
