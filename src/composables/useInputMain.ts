import { ref, watch } from "vue";
import { useMessage } from "./useMessage";
import { Palette } from "../types/palette";

const { afficherMessage } = useMessage();

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
  const rgb = resultat;
  r.value = rgb.r;
  g.value = rgb.g;
  b.value = rgb.b;
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

export function useInputMain() {
  return {
    codeHex,
    displayValue,
    r,
    g,
    b,
    paletteCourante,
    estFonce,
    onSliderChange,
    onInput,
    onChange,
    copierAuPressePapier,
    calculerValeursRGB,
  };
}

export function calculerCodeHex(r: number, g: number, b: number) {
  return [r, g, b]
    .map((c) => c.toString(16).padStart(2, "0"))
    .join("")
    .toUpperCase();
}

function calculerValeursRGB(codeHex: string) {
  const cleanHex = codeHex.replace("#", "");
  const num = parseInt(cleanHex, 16);
  return {
    r: (num >> 16) & 255,
    g: (num >> 8) & 255,
    b: num & 255,
  };
}

function isValidHex(value: string): boolean {
  return /^[0-9A-Fa-f]{6}$/.test(value);
}

function estFonce(r: number, g: number, b: number): boolean {
  const luminance = 0.299 * r + 0.587 * g + 0.114 * b;
  return luminance <= 127;
}
