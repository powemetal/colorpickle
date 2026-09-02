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
  <div class="container flex flex-1 flex-col">
    <div class="flex-1" />
    <div class="flex">
      <div class="flex flex-col">
        <Slider v-model="r" @change="onSliderChange" />
        <Slider v-model="g" @change="onSliderChange" />
        <Slider v-model="b" @change="onSliderChange" />
      </div>
      <div class="">
        <span>#</span>
        <input
          maxlength="6"
          v-model.trim="displayValue"
          @change="onChange"
          @input="onInput"
          @dblclick="copierAuPressePapier(`#${displayValue}`)"
        />
      </div>
    </div>
  </div>
</template>
