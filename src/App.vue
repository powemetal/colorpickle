<script setup lang="ts">
import "@fortawesome/fontawesome-free/css/all.min.css";
import MessageFooter from "./components/MessageFooter.vue";
import Navigation from "./components/Navigation.vue";
import useColorPicker from "./composables/useColorPicker.ts";
import { computed, onMounted, onUnmounted } from "vue";
import { useRoute } from "vue-router";
import { usePalette } from "./composables/usePalette.ts";

const route = useRoute();
const { initMainListener, cleanMainListener } = useColorPicker();
const { chargerPalettes } = usePalette();

// Détecte si la fenêtre courante est un overlay ou la fenêtre principale
const isOverlay = computed(() => route.path === "/overlay");

onMounted(() => {
  if (!isOverlay.value) {
    initMainListener();
  }
});

onMounted(() => {
chargerPalettes();
});

onUnmounted(() => {
  if (!isOverlay.value) {
    cleanMainListener();
  }
});
</script>

<template>
  <RouterView v-if="isOverlay" />
  <div
    v-else
    class="flex flex-1 flex-col h-screen"
    style="background-color: var(--color-bg)"
  >
    <Navigation />
    <main class="flex flex-1 flex-col overflow-y-auto">
      <RouterView />
    </main>
    <MessageFooter />
  </div>
</template>

