import { Couleur } from "./couleur"

export interface Palette {
  id: number;
  nom: string;
  couleurs: Couleur[];
  createdAt: Date;
}
