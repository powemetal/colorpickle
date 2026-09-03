<script setup lang="ts">
const valeur = defineModel<number>();

interface Props {
  couleur?: keyof typeof couleurClasses;
}
const props = withDefaults(defineProps<Props>(), {
  couleur: "rouge",
});
const couleurClasses = {
  rouge: [
    "[&::-webkit-slider-runnable-track]:appearance-none",
    "[&::-webkit-slider-runnable-track]:h-2",
    "[&::-webkit-slider-runnable-track]:rounded-full",
    "[&::-webkit-slider-runnable-track]:bg-red-500",
    "[&::-webkit-slider-runnable-track]:border",
    "[&::-webkit-slider-runnable-track]:border-gray-500",

    "[&::-webkit-slider-thumb]:appearance-none",
    "[&::-webkit-slider-thumb]:w-4",
    "[&::-webkit-slider-thumb]:h-4",
    "[&::-webkit-slider-thumb]:rounded-full",
    "[&::-webkit-slider-thumb]:bg-white",
    "[&::-webkit-slider-thumb]:border",
    "[&::-webkit-slider-thumb]:border-gray-500",
    "[&::-webkit-slider-thumb]:mt-[-5px]",
    "[&::-webkit-slider-thumb]:cursor-pointer",
  ].join(" "),

  bleu: [
    "[&::-webkit-slider-runnable-track]:appearance-none",
    "[&::-webkit-slider-runnable-track]:h-2",
    "[&::-webkit-slider-runnable-track]:rounded-full",
    "[&::-webkit-slider-runnable-track]:bg-blue-500",
    "[&::-webkit-slider-runnable-track]:border",
    "[&::-webkit-slider-runnable-track]:border-gray-500",

    "[&::-webkit-slider-thumb]:appearance-none",
    "[&::-webkit-slider-thumb]:w-4",
    "[&::-webkit-slider-thumb]:h-4",
    "[&::-webkit-slider-thumb]:rounded-full",
    "[&::-webkit-slider-thumb]:bg-white",
    "[&::-webkit-slider-thumb]:border",
    "[&::-webkit-slider-thumb]:border-gray-500",
    "[&::-webkit-slider-thumb]:mt-[-4px]",
    "[&::-webkit-slider-thumb]:cursor-pointer",
  ].join(" "),

  vert: [
    "[&::-webkit-slider-runnable-track]:appearance-none",
    "[&::-webkit-slider-runnable-track]:h-2",
    "[&::-webkit-slider-runnable-track]:rounded-full",
    "[&::-webkit-slider-runnable-track]:bg-green-500",
    "[&::-webkit-slider-runnable-track]:border",
    "[&::-webkit-slider-runnable-track]:border-gray-500",

    "[&::-webkit-slider-thumb]:appearance-none",
    "[&::-webkit-slider-thumb]:w-4",
    "[&::-webkit-slider-thumb]:h-4",
    "[&::-webkit-slider-thumb]:rounded-full",
    "[&::-webkit-slider-thumb]:bg-white",
    "[&::-webkit-slider-thumb]:border",
    "[&::-webkit-slider-thumb]:border-gray-500",
    "[&::-webkit-slider-thumb]:mt-[-4px]",
    "[&::-webkit-slider-thumb]:cursor-pointer",
  ].join(" "),
} as const;

const emit = defineEmits(["change"]);

function verifierCharValide(e: Event) {
  const target = e.target as HTMLInputElement;
  let val = Number(target.value);
  val = Math.min(255, Math.max(0, val));
  valeur.value = val;
}
</script>

<template>
  <div class="flex w-full gap-2">
    <input
      :class="[
        'w-full',
        'h-4', 
        'my-auto',
        'bg-transparent',
        'appearance-none',
        'cursor-pointer',
        couleurClasses[props.couleur],
      ]"
      type="range"
      min="0"
      max="255"
      v-model.number="valeur"
      @input="emit('change', valeur)"
    />
    <input
      class="text-center text-gray-300 w-15 bg-gray-700/20 hover:bg-gray-600/40 transition rounded-full no-spinner p-1 border-none outline-none"
      type="number"
      min="0"
      max="255"
      v-model.number="valeur"
      @input="verifierCharValide"
      @change="emit('change', valeur)"
    />
  </div>
</template>
