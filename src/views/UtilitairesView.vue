<script setup lang="ts">
import { useInputMain } from "../composables/useInputMain";
import { useUtils } from "../composables/useUtils";

const { couleurEstFonce, copierAuPressePapier } = useInputMain();

const { couleurComplementaire, couleursTriade } = useUtils();
</script>

<template>
  <div class="flex flex-col w-full py-5 px-12 gap-4">
    <h2
      :style="{
        color: couleurEstFonce
          ? 'var(--color-text-white)'
          : 'var(--color-text-dark)',
      }"
    >
      Couleur complémentaire:
    </h2>
    <div
      class="complementaire flex w-full rounded-4xl min-h-24 border-solid border-4 font-bold mb-4 hover:scale-[1.02] transition"
      :style="{
        backgroundColor: '#' + couleurComplementaire.hex,
        borderColor: couleurEstFonce
          ? 'var(--color-text-dark)'
          : 'var(--color-text-white)',
      }"
    >
      <button
        @click="copierAuPressePapier(couleurComplementaire.hex)"
        class="flex grow justify-center items-center cursor-pointer"
        :style="{
          color:couleurEstFonce ? 'black' : 'white',
        }"
      >
        <span>#</span>{{ couleurComplementaire.hex }}
      </button>
    </div>

    <h2
      :style="{
        color: couleurEstFonce
          ? 'var(--color-text-white)'
          : 'var(--color-text-dark)',
      }"
    >
      Reste de la triade:
    </h2>
    <div class="flex justify-between gap-8 w-full">
      <div
        class="triade1 flex w-full rounded-4xl min-h-24 border-solid border border-4 font-bold hover:scale-[1.02] transition"
        :style="{
          backgroundColor: '#' + couleursTriade.triade1.hex,
          borderColor: couleurEstFonce
            ? 'var(--color-text-light)'
            : 'var(--color-text-dark)',
        }"
      >
        <button
          @click="copierAuPressePapier(couleursTriade.triade1.hex)"
          class="flex grow justify-center items-center cursor-pointer"
          :style="{
            color: couleurEstFonce ? 'white' : 'black',
          }"
        >
          <span>#</span>{{ couleursTriade.triade1.hex }}
        </button>
      </div>

      <div class="flex w-full rounded-4xl">
        <div
          class="triade2 flex w-full rounded-4xl min-h-24 border-solid border border-4 font-bold hover:scale-[1.02] transition"
          :style="{
            backgroundColor: '#' + couleursTriade.triade2.hex,
            borderColor: couleurEstFonce
              ? 'var(--color-text-light)'
              : 'var(--color-text-dark)',
          }"
        >
          <button
            @click="copierAuPressePapier(couleursTriade.triade2.hex)"
            class="flex grow justify-center items-center cursor-pointer"
            :style="{
              color: couleurEstFonce ? 'white' : 'black',
            }"
          >
            <span>#</span>{{ couleursTriade.triade2.hex }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
