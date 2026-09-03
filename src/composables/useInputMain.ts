import { ref, watch } from "vue";
import { useMessage } from "./useMessage";
import { Palette } from "../types/palette";
import useColorPicker from "./useColorPicker";

const { afficherMessage } = useMessage();
const { ouvrirPipette } = useColorPicker();

export function useInputMain() {
  const codeHex = ref("415F42");
  const displayValue = ref(codeHex.value);
  const r = ref(65);
  const g = ref(95);
  const b = ref(66);
  const paletteCourante: Palette = {
    id: 1,
    nom: "Design",
    couleurs: [],
    createdAt: new Date(Date.now()),
  };

  watch(codeHex, (newValue) => {
    document.documentElement.style.setProperty("--color-bg", `#${newValue}`);
  });

  watch(codeHex, (val) => {
    displayValue.value = val;
  });

  function onSliderChange() {
    codeHex.value = calculerCodeHex(r.value, g.value, b.value);
  }

  function onHexChange() {
    if (!isValidHex(codeHex.value)) return;
    const resultat = calculerValeursRGB(codeHex.value);
    if (!resultat) return;
    const [newR, newG, newB] = resultat;
    r.value = newR;
    g.value = newG;
    b.value = newB;
  }

  function onInput(e: Event) {
    const target = e.target as HTMLInputElement;
    let newValue = target.value.toUpperCase();

    newValue = newValue.replace(/[^0-9A-F]/g, "");

    displayValue.value = newValue;
    target.value = newValue;
  }

  function onChange() {
    if (displayValue.value.length === 6 && isValidHex(displayValue.value)) {
      codeHex.value = displayValue.value;
      onHexChange();
    } else {
      displayValue.value = codeHex.value;
    }
  }

  async function copierAuPressePapier(text: string) {
    try {
      await navigator.clipboard.writeText(text);
      afficherMessage("Code hex copié avec succès !");
    } catch (error) {
      console.error("Erreur lors de la copie du texte :", error);
    }
  }

  async function recupererCouleurPick() {
    const couleur = await ouvrirPipette();
    if (!couleur) return;
    codeHex.value = couleur;
  }

  return {
    displayValue,
    r,
    g,
    b,
    paletteCourante,
    onSliderChange,
    onInput,
    onChange,
    copierAuPressePapier,
    recupererCouleurPick,
  };
}

function calculerCodeHex(r: number, g: number, b: number) {
  return [r, g, b]
    .map((c) => c.toString(16).padStart(2, "0"))
    .join("")
    .toUpperCase();
}

function calculerValeursRGB(codeHex: string) {
  return codeHex.match(/.{1,2}/g)!.map((c) => parseInt(c, 16));
}

function isValidHex(value: string): boolean {
  return /^[0-9A-Fa-f]{6}$/.test(value);
}
