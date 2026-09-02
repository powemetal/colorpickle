<script setup lang="ts">
import { useMessage } from "../composables/useMessage";

const { messageFooter, estMessageDErreur } = useMessage();
</script>

<template>
  <div class="h-2 m-2">
    <p
      :class="[
        estMessageDErreur ? 'text-red-500' : 'text-green-500',
        'font-bold text-center',
      ]"
      v-if="messageFooter"
    >
      {{ messageFooter }}
    </p>
  </div>
</template>
