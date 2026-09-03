<script setup lang="ts">
import Slider from "../components/Slider.vue";
import {
  copierAuPressePapier,
  useColorPicker,
} from "../composables/useInputMain.ts";

const { displayValue, r, g, b, onSliderChange, onInput, onChange } =
  useColorPicker();
</script>

<template>
  <div class="container mx-auto flex flex-1 flex-col">
    <div class="flex mt-auto m-4 gap-4">
      <div class="flex flex-1 flex-col gap-2 items-center">
        <Slider v-model="r" @input="onSliderChange" couleur="rouge" />
        <Slider v-model="g" @input="onSliderChange" couleur="vert"/>
        <Slider v-model="b" @input="onSliderChange" couleur="bleu"/>
      </div>
      <div class="flex flex-col justify-center w-40">
        <div
          class="flex w-full justify-center text-gray-300 bg-gray-700/20 hover:bg-gray-600/40 transition rounded-full px-3 py-1"
        >
          <span class="text-lg">#</span>
          <input
            class="w-20 bg-transparent outline-none border-none text-lg"
            maxlength="6"
            v-model.trim="displayValue"
            @change="onChange"
            @input="onInput"
            @dblclick="copierAuPressePapier(`#${displayValue}`)"
          />
        </div>
        <div class="flex items-center mt-4 mx-auto w-full min-w-0 gap-2">
        <span
          class="flex-1 min-w-0 text-center text-gray-300 bg-gray-800/40 rounded-full px-3 py-1 truncate"
          >Palette</span
        >
        <button class=" h-8 w-8 bg-gray-700/20 hover:bg-gray-600/40 transition rounded-full px-2">+</button>
        </div>
      </div>
    </div>
  </div>
</template>
