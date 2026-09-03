<script setup lang="ts">

import {ref, computed } from "vue";
import { useColorPicker, copierAuPressePapier } from "../composables/useInputMain";

// const { couleurPicked } = useColorPicker()
// const { triade1Value, r, g, b} = 
// const { triade2Value, r, g, b} =
// const { complementaireValue, r, g ,b} = 

// valeurs test
const couleurTest1 = "196788";
const couleurTest2 = "AAA788";
const couleurTest3 = "19BBB8";
const estFonce = false;


</script>

<template>
    <div class="flex flex-col w-full py-5 px-12 gap-4">

        <h2>Couleur complémentaire:</h2>
        <div class="complementaire flex w-full rounded-4xl min-h-28 border-solid border-gray-500 border border-4 font-bold mb-4 hover:scale-[1.02] transition" 
            :style="{ 
            backgroundColor: '#' + couleurTest1,
            borderColor: estFonce ? 'var(--color-text-light)' : 'var(--color-text-dark)'}">

            <button @click="copierAuPressePapier(couleurTest1)" class="flex grow justify-center items-center">
                <span>#</span>code Hexa complementaire
            </button>

        </div>

        <h2>Reste de la triade:</h2>
        <div class="flex justify-between gap-8 w-full">
            <div class="triade1 flex w-full rounded-4xl min-h-28 border-solid border-gray-500 border border-4 font-bold hover:scale-[1.02] transition" 
                    :style="{ 
                    backgroundColor: '#' + couleurTest2,
                    borderColor: estFonce ? 'var(--color-text-light)' : 'var(--color-text-dark)'}">

                <button @click="copierAuPressePapier(couleurTest2)" class="flex grow justify-center items-center">
                    <span>#</span>code Hexa triade 1
                </button>

            </div>
            
            <div class="flex w-full rounded-4xl min-h-30">
                <div class="triade2 flex w-full rounded-4xl min-h-28 border-solid border-gray-500 border border-4 font-bold hover:scale-[1.02] transition"
                    :style="{ 
                    backgroundColor: '#' + couleurTest3,
                    borderColor: estFonce ? 'var(--color-text-light)' : 'var(--color-text-dark)'}">

                    <button @click="copierAuPressePapier(couleurTest3)" class="flex grow justify-center items-center">
                        <span>#</span>code Hexa triade 2 
                    </button>

                </div>
            </div>

        </div>
    </div>


</template>