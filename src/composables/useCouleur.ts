import { Couleur } from "../types/couleur.ts";



export function useCouleur() {

  function estFonce(r: number, g:number, b:number): boolean {
    const luminance = 0.299 * r + 0.587 * g + 0.114 * b;
    return luminance <= 127;
  }


  function creationCouleur(nomNouvelleCouleur: string, r: number, g: number, b:number, displayValue: string): Couleur {
    return {
      id: Date.now(),
      nom: nomNouvelleCouleur,
      valeurRouge: r,
      valeurVert: g,
      valeurBleu: b,
      codeHex: "#" + displayValue,
      createdAt: new Date(),
    };
  }

  return {
    estFonce,
    creationCouleur
  }

}