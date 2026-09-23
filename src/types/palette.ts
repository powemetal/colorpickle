import { Couleur } from "./couleur"

export interface Palette {
  id: string;
  nom: string;
  couleurs: Couleur[];
  createdAt: Date;
}
