import { Couleur } from "../types/couleur.ts";



export function useCouleur() {

  function estFonce(r: number, g:number, b:number): boolean {
    const luminance = 0.299 * r + 0.587 * g + 0.114 * b;
    return luminance <= 127;
  }

  // ne retourne plus un Couleur complet, seulement un partiel des informations d'où le Omit
  // auparavant on retournait un id et un createdAt créé par le frontend qui ne sont pas nécessaires puisque le 
  // backend s'en charge deja
  function creationCouleur(nomNouvelleCouleur: string, r: number, g: number, b:number, displayValue: string): Omit<Couleur, "id" | "createdAt"> {
    return {
      nom: nomNouvelleCouleur,
      valeurRouge: r,
      valeurVert: g,
      valeurBleu: b,
      codeHex: "#" + displayValue,
    };
  }

  return {
    estFonce,
    creationCouleur
  }

}