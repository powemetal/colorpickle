<script setup lang="ts">
const valeur = defineModel<number>();
const emit = defineEmits(["change"]);

function verifierCharValide(e: Event) {
  const target = e.target as HTMLInputElement;
  let val = Number(target.value);
  val = Math.min(255, Math.max(0, val));
  valeur.value = val;
}
</script>

<template>
  <div class="flex gap-2">
    <input
      type="range"
      min="0"
      max="255"
      v-model.number="valeur"
      @change="emit('change', valeur)"
    />
    <input
      type="number"
      min="0"
      max="255"
      v-model.number="valeur"
      @input="verifierCharValide"
      @change="emit('change', valeur)"
    />
  </div>
</template>
