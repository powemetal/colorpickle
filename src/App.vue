<script setup lang="ts">
import '@fortawesome/fontawesome-free/css/all.min.css'
import { ref } from "vue";
import MessageFooter from "./components/MessageFooter.vue";
import Navigation from "./components/Navigation.vue";
import { useMessage } from "./composables/useMessage.ts";
</script>

<template>
  <div class="flex flex-1 flex-col">
    <Navigation />
    <main class="flex flex-1 flex-col">
      <RouterView />
    </main>
    <MessageFooter/>
  </div>
</template>
