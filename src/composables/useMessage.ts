import { ref } from "vue";

const messageFooter = ref("");
const estMessageDErreur = ref(false);
const messageVisible = ref(false);

export function useMessage() {
  function afficherMessage(texte: string, estErreur = false, dureeMs = 4000) {
    estMessageDErreur.value = estErreur;
    messageFooter.value = texte;
    messageVisible.value = true;

    if (dureeMs > 0) {
      setTimeout(() => {
        if (messageFooter.value === texte) {
          messageVisible.value = false;
        }
      }, dureeMs);
    }
  }

  return {
    messageFooter,
    estMessageDErreur,
    afficherMessage,
    messageVisible,
  };
}
