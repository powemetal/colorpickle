<script setup lang="ts">
import logo from "@/assets/colorPickleLogo.png";
import { useMessage } from "../composables/useMessage";
import { useInputMain } from "../composables/useInputMain";

const { recupererCouleurPick } = useInputMain();
const estFonce = true;

const messageSauvegarde = "Données sauvegardées!";
</script>

<template>
  <nav>
    <div class="flex transition">
      <div
        class="bg-black/20 rounded-full w-24 h-24 min-w-24 flex flex-1 justify-center items-center m-4 hover:border-4 hover:border-solid hover:border-black/10"
      >
        <RouterLink :to="{ name: 'accueil' }">
          <img class=" w-16 mix-blend-overlay" :src="logo" alt="Logo"/>
        </RouterLink>
      </div>


      <div class="flex items-start justify-end m-4 w-full">
        <div class="flex">
          <div
            class="flex bg-black/20 rounded-3xl w-18 h-12 justify-center items-center mx-2 hover:border-4 hover:border-solid hover:border-black/10"
          >
            <button @click="recupererCouleurPick">
              <i class=" fa-solid fa-eye-dropper fa-2xl mix-blend-overlay"
              :style="{
                color: estFonce
                  ? 'white'
                  : 'black',
              }"></i>
            </button>
          </div>
          <div
            class="flex bg-black/20 rounded-3xl w-18 h-12 justify-center items-center  mx-2 hover:border-4 hover:border-solid hover:border-black/10"
          >
            <RouterLink :to="{ name: 'palettes' }"
              >
              <i class=" fa-solid fa-swatchbook fa-2xl mix-blend-overlay "
                :style="{
                  color: estFonce
                    ? 'white'
                    : 'black',
                }"
              ></i>
            </RouterLink>
          </div>
          <div
            class="flex bg-black/20 rounded-3xl w-18 h-12 justify-center items-center mx-2  hover:border-4 hover:border-solid hover:border-black/10"
          >
            <button @click="useMessage().afficherMessage(messageSauvegarde)">
              <i class=" fa-solid fa-floppy-disk mix-blend-overlay fa-2xl"
              :style="{
                color: estFonce
                  ? 'white'
                  : 'black',
              }">
              </i>
            </button>
          </div>
          <div
            class="flex bg-black/20 rounded-3xl w-18 h-12 justify-center items-center mx-2  hover:border-4 hover:border-solid hover:border-black/10"
          >
            <RouterLink :to="{ name: 'utilitaires' }">
                <i class="fa-solid fa-palette mix-blend-overlay fa-2xl "
                :style="{
                  color: estFonce
                    ? 'white'
                    : 'black',
                }"></i>
            </RouterLink>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<style></style>
