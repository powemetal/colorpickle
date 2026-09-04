<script setup lang="ts">
import { useInputMain } from "../composables/useInputMain";
import { useMessage } from "../composables/useMessage";

const { messageFooter, estMessageDErreur, messageVisible } = useMessage();
const {
  r,
  g,
  b,
  estFonce,
} = useInputMain();

</script>

<template>
  <div class="h-4 m-2 mb-4 mx-4 font-bold text-center">
    <p class="transition-opacity duration-400"
      :class="{
        'text-red-400': estMessageDErreur && estFonce(r,g,b),
        'text-red-700': estMessageDErreur && !estFonce(r,g,b),
        'text-green-400': !estMessageDErreur && estFonce(r,g,b),
        'text-green-700': !estMessageDErreur && !estFonce(r,g,b),
        'opacity-100': messageVisible,
        'opacity-0': !messageVisible,
      }"
    >
      {{ messageFooter }}
    </p>
  </div>
</template>
