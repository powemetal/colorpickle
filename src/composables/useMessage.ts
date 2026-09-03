import { ref } from "vue";

const messageFooter = ref("");
const estMessageDErreur = ref(false);

export function useMessage() {
  function afficherMessage(texte: string, estErreur = false, dureeMs = 4000) {
    estMessageDErreur.value = estErreur;
    messageFooter.value = texte;

    if (dureeMs > 0) {
      setTimeout(() => {
        if (messageFooter.value === texte) {
          messageFooter.value = "";
        }
      }, dureeMs);
    }
  }

  return {
    messageFooter,
    estMessageDErreur,
    afficherMessage,
  };
}
