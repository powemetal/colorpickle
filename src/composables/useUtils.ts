import { computed } from "vue";
import { useInputMain, calculerCodeHex } from "./useInputMain"


export function useUtils() {

    const {
    r,
    g,
    b,
    } = useInputMain();
    
    
    // La complémentaire est la couleur "inverse", on la trouve en soustraiant la valeur des canaux à 255.
    const couleurComplementaire = computed(() => {
        const compR = 255 - r.value;
        const compG = 255 - g.value;
        const compB = 255 - b.value;
        const compHex = calculerCodeHex(compR, compG, compB);
        return { r: compR, g: compG, b: compB, hex: compHex };
    });
    
    // La triade de la couleur 0/0/255 est 0/255/0 et 255/0/0 donc on n'a qu'à permutter les valeurs trouver les deux autres couleurs
    const couleursTriade = computed(() => {
        const triade1 = { r: b.value, g: r.value, b: g.value };
        const triade2 = { r: g.value, g: b.value, b: r.value };
        const triade1Hex = calculerCodeHex(triade1.r, triade1.g, triade1.b);
        const triade2Hex = calculerCodeHex(triade2.r, triade2.g, triade2.b);

        return {
        triade1: { ...triade1, hex: triade1Hex },
        triade2: { ...triade2, hex: triade2Hex },
        };
    });

    return { couleurComplementaire, couleursTriade };
}

