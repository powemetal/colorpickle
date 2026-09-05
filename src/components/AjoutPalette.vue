<script setup lang="ts">
        import { ref } from "vue"
        const nouveauNom = ref("")

    const props = defineProps<{
        ajouterPalette: (nom: string) => void
    }>()

    const soumettrePalette = () => {
        props.ajouterPalette(nouveauNom.value)
        nouveauNom.value = ""
    }

</script>

<template>
    <div class="p-3 flex flex-col items-center justify-center text-center h-full ">
        <h1 class="text-center text-2xl mb-3">Ajouter une palette</h1>
            
        <h3 class="text-center">Entrez le nom de la nouvelle palette</h3>
        <input 
            type="text" 
            class="border text-white my-3"  
            v-model="nouveauNom"
            @keyup.enter="soumettrePalette"
        />

        <button
            class="px-4 py-2 rounded-lg 
                    bg-white/40 backdrop-blur-sm 
                    text-black font-semibold
                    hover:bg-white/60 hover:shadow-lg
                    transition-all duration-200"
            @click="soumettrePalette"
                        
        >
            Ajouter
        </button>
    </div>
</template>