<script setup lang="ts">
import { useMessage } from "../composables/useMessage";

const { messageFooter, estMessageDErreur } = useMessage();
</script>

<template>
  <div class="h-4 m-2 mb-4 mx-4">
    <p
      :class="[
        estMessageDErreur ? 'text-red-500' : 'text-green-500',
        'font-bold text-center',
      ]"
    >
      {{ messageFooter }}
    </p>
  </div>
</template>
