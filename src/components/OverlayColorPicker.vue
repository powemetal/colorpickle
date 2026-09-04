<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";
import useColorPicker from "../composables/useColorPicker";

const {
  mouseX,
  mouseY,
  currentHex,
  onMouseMove,
  onMouseLeave,
  onClick,
  onKeyDown,
  isHovered,
} = useColorPicker();

onMounted(() => {
  window.focus();
  window.addEventListener("keydown", onKeyDown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", onKeyDown);
});

// Dimensions estimées du badge avec marge de sécurité
const BADGE_WIDTH = 130;
const BADGE_HEIGHT = 45;
const OFFSET_X = 20;
const OFFSET_Y = 20;

const badgePosition = computed(() => {
  const winWidth = window.innerWidth;
  const winHeight = window.innerHeight;

  let x = mouseX.value + OFFSET_X;
  // Si ça dépasse à droite, on bascule à gauche de la souris
  if (x + BADGE_WIDTH > winWidth) {
    x = mouseX.value - BADGE_WIDTH - OFFSET_X;
  }
  // Ne jamais sortir à gauche ou à droite
  x = Math.max(8, Math.min(x, winWidth - BADGE_WIDTH - 8));

  let y = mouseY.value + OFFSET_Y;
  // Si ça dépasse en bas, on bascule au-dessus de la souris
  if (y + BADGE_HEIGHT > winHeight) {
    y = mouseY.value - BADGE_HEIGHT - OFFSET_Y;
  }
  // Ne jamais sortir en haut ou en bas
  y = Math.max(8, Math.min(y, winHeight - BADGE_HEIGHT - 8));

  return { x, y };
});
</script>

<template>
  <div
    class="overlay-container"
    @mousemove="onMouseMove"
    @mouseleave="onMouseLeave"
    @click="onClick"
  >
    <div
      v-if="isHovered"
      class="preview-badge"
      :style="{
        transform: `translate3d(${badgePosition.x}px, ${badgePosition.y}px, 0)`,
      }"
    >
      <div class="color-box" :style="{ backgroundColor: `#${currentHex}` }" />
      <span class="hex-text">#{{ currentHex }}</span>
    </div>
  </div>
</template>

<style scoped>
.overlay-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  cursor:
    url("/pickle-pipette.svg") 1 31,
    crosshair;
  user-select: none;
  overflow: hidden;
  border: none !important;
  outline: none !important;
  z-index: 999999;
}

.preview-badge {
  position: fixed;
  top: 0;
  left: 0;
  pointer-events: none;
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: rgba(15, 15, 15, 0.9);
  padding: 6px 10px;
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.color-box {
  width: 16px;
  height: 16px;
  border-radius: 4px;
}

.hex-text {
  color: white;
  font-family: monospace;
  font-size: 13px;
  font-weight: bold;
}
</style>
