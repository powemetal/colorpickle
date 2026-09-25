<script setup lang="ts">
import logo from "@/assets/colorPickleLogo.png";
import { useMessage } from "../composables/useMessage";
import { useInputMain } from "../composables/useInputMain";
import useColorPicker from "../composables/useColorPicker";
import { useRoute } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";



const { champRecherche } = useInputMain();
const { afficherMessage } = useMessage();

const { estFonce, r, g, b } = useInputMain();
const route = useRoute();
const { ouvrirColorPicker } = useColorPicker();

async function exporterDonnees() {
  try {
    const chemin = await save({
      defaultPath: "palettes.json",
      filters: [{name: "JSON", extensions: ["json"]}],
    });
    if (!chemin) return;

    await invoke<void>("exporter_donnees", {cheminDestination: chemin});
    afficherMessage("Palettes exportées!");
  } catch (error) {
    console.error(error);
    afficherMessage("Erreur lors de l'export.", true);
  }
}

</script>

<template>
  <nav>
    
    <div class="flex transition">
      <RouterLink :to="{ name: 'accueil' }">
        <div
          class="rounded-full w-24 h-24 min-w-24 flex flex-1 justify-center items-center m-4 hover:border-4 hover:border-solid hover:border-black/10"
          :style="{
            backgroundColor: estFonce(r, g, b)
              ? 'rgba(256,256,256,0.2)'
              : 'rgba(0,0,0,0.2)',
          }"
        >
          <img class="w-16 mix-blend-overlay" :src="logo" alt="Logo" />
        </div>
      </RouterLink>

      <div class="flex items-start justify-end m-4 w-full">
        
        <div>
          <div class="flex">
            <button
              @click="ouvrirColorPicker"
              class="flex bg-black/20 cursor-pointer rounded-3xl w-18 h-12 justify-center items-center mx-2 hover:border-4 hover:border-solid hover:border-black/10"
              :style="{
                backgroundColor: estFonce(r, g, b)
                  ? 'rgba(256,256,256,0.2)'
                  : 'rgba(0,0,0,0.2)',
              }"
            >
              <i
                class="fa-solid fa-eye-dropper fa-2xl mix-blend-overlay"
                :style="{ color: 'white' }"
              >
              </i>
            </button>

            <RouterLink :to="{ name: 'palettes' }">
              <div
                class="flex bg-black/20 rounded-3xl w-18 h-12 justify-center items-center mx-2 hover:border-4 hover:border-solid hover:border-black/10"
                :style="{
                  backgroundColor: estFonce(r, g, b)
                    ? 'rgba(256,256,256,0.2)'
                    : 'rgba(0,0,0,0.2)',
                }"
              >
                <i
                  class="fa-solid fa-swatchbook fa-2xl mix-blend-overlay"
                  :style="{
                    color: 'white',
                  }"
                ></i>
              </div>
            </RouterLink>

            <button
              @click="exporterDonnees"
              class="flex bg-black/20 cursor-pointer rounded-3xl w-18 h-12 justify-center items-center mx-2 hover:border-4 hover:border-solid hover:border-black/10"
              :style="{
                backgroundColor: estFonce(r, g, b)
                  ? 'rgba(256,256,256,0.2)'
                  : 'rgba(0,0,0,0.2)',
              }"
            >
              <i
                class="fa-solid fa-floppy-disk mix-blend-overlay fa-2xl cursor-pointer"
                :style="{ color: 'white' }"
              >
              </i>
            </button>

            <RouterLink :to="{ name: 'utilitaires' }">
              <div
                class="flex bg-black/20 rounded-3xl w-18 h-12 justify-center items-center mx-2 hover:border-4 hover:border-solid hover:border-black/10"
                :style="{
                  backgroundColor: estFonce(r, g, b)
                    ? 'rgba(256,256,256,0.2)'
                    : 'rgba(0,0,0,0.2)',
                }"
              >
                <i
                  class="fa-solid fa-palette mix-blend-overlay fa-2xl"
                  :style="{
                    color: 'white',
                  }"
                ></i>
              </div>
            </RouterLink>
          </div>
          <div class="mt-3 flex justify-center ">
            <input 
            type="text"
            v-if="route.name === 'palettes'"
            v-model="champRecherche"
            placeholder="Chercher une couleur"
            class="border mt-2 text-center rounded-lg min-w-80 text-center p-1"
            :style="{
                  color: estFonce(r, g, b)
                    ? 'rgba(256,256,256)'
                    : 'rgba(0,0,0)',
                }"
            />
          </div>
        </div>

      </div>
    </div>

  </nav>
</template>

<style></style>
