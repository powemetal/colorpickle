<script setup lang="ts">
    import { ref } from "vue"
    import { useMessage } from "../composables/useMessage.ts";


    const nouveauNom = ref("")
    const { afficherMessage } = useMessage();

    const props = defineProps<{
        renommerPalette: (paletteId: string, nom: string) => void
        paletteChoisieId: string
    }>()

    const soumettrePalette = async () => {
        await props.renommerPalette(props.paletteChoisieId, nouveauNom.value)
        nouveauNom.value = ""
        emit('renommer-complete');
    }

    const emit = defineEmits<{
        (e: 'renommer-complete'): void
    }>();

</script>

<template>
    <div class="p-3 flex flex-col items-center justify-center text-center h-full ">
        <h1 class="text-center text-2xl mb-3">Renommer une palette</h1>
            
        <h3 class="text-center">Entrez le nouveau nom de votre palette</h3>
        <input 
            type="text" 
            class="border text-white my-3 rounded-lg text-center p-1"  
            maxlength="15"
            v-model="nouveauNom"
            @keyup.enter="nouveauNom
            ? soumettrePalette()
            : afficherMessage('Veuillez entrer un nom de palette!', true);
            "
        />
        <div class="flex gap-2">
        <button
            class="px-4 py-2 rounded-lg 
                    bg-white/40 backdrop-blur-sm 
                    text-black font-semibold
                    hover:bg-white/60 hover:shadow-lg
                    transition-all duration-200"
            @click="nouveauNom
            ? soumettrePalette()
            : afficherMessage('Veuillez entrer un nom de palette!', true);
            "
                        
        >
            Renommer
        </button>
                <button
            class="px-4 py-2 rounded-lg 
                    bg-white/40 backdrop-blur-sm 
                    text-black font-semibold
                    hover:bg-white/60 hover:shadow-lg
                    transition-all duration-200"
            @click="emit('renommer-complete')"
                        
        >
            Annuler
        </button>
        </div>
    </div>
</template>