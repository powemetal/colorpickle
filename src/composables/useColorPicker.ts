import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useMessage } from "./useMessage";
import { useInputMain } from "./useInputMain";

const { afficherMessage } = useMessage();
const { codeHex, r, g, b, calculerValeursRGB } = useInputMain();

export default function useColorPicker() {
  const mouseX = ref(0);
  const mouseY = ref(0);
  const currentHex = ref("FFFFFF");
  const isHovered = ref(false);

  async function onMouseMove(e: MouseEvent) {
    isHovered.value = true;
    mouseX.value = e.clientX;
    mouseY.value = e.clientY;

    try {
      const hex = await invoke<string>("read_pixel_at");
      currentHex.value = hex;
    } catch (err) {
      console.error("Erreur read_pixel_at :", err);
    }
  }

  function onMouseLeave() {
    isHovered.value = false;
  }

  async function onClick() {
    try {
      await invoke("confirm_color_pick", {
        hex: currentHex.value,
      });
    } catch (err) {
      console.error("Erreur confirm_color_pick :", err);
    }
  }

  async function onKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      try {
        await invoke("cancel_color_pick");
      } catch (err) {
        console.error("Erreur cancel_color_pick :", err);
      }
    }
  }

  async function ouvrirColorPicker() {
    try {
      await invoke("start_color_pick");
    } catch (error) {
      console.error("Erreur d'ouverture des overlays :", error);
      afficherMessage("Impossible d'activer la pipette.", true);
    }
  }

  // Écoute de la sélection finale envoyée par l'overlay
  let unlistenColorSelected: UnlistenFn | null = null;

  async function initMainListener() {
    unlistenColorSelected = await listen<string>("color-selected", (event) => {
      const hex = event.payload;
      codeHex.value = hex;

      const rgb = calculerValeursRGB(hex);
      r.value = rgb.r;
      g.value = rgb.g;
      b.value = rgb.b;

      afficherMessage("Couleur récupérée avec succès !");
    });
  }

  function cleanMainListener() {
    if (unlistenColorSelected) {
      unlistenColorSelected();
    }
  }

  return {
    ouvrirColorPicker,
    initMainListener,
    cleanMainListener,
    mouseX,
    mouseY,
    currentHex,
    isHovered,
    onMouseMove,
    onMouseLeave,
    onClick,
    onKeyDown,
  };
}
