# ColorPickle - Gestionnaire de Palettes de Couleurs

Application de bureau developpee avec Tauri 2 et Vue 3 permettant de creer, personnaliser, convertir et gerer des palettes de couleurs.

Projet realise pour le travail pratique # 2 (TP2) dans le cadre du cours Développement d'application (bureau)  
Présenté à Mme Lilia Ould Hocine  
(AEC en Developpement Web / Programmation - College de Maisonneuve).  
<br>
![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri)
![Vue 3](https://img.shields.io/badge/Vue.js-3-42b883?logo=vuedotjs&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-5-blue?logo=typescript)
![Vite](https://img.shields.io/badge/Vite-5.0-646CFF?logo=vite&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-backend-orange?logo=rust)
![npm](https://img.shields.io/badge/npm-10-red?logo=npm)

<br>

# 1. STACK TECHNIQUE


* Framework Desktop : Tauri 2 (avec backend minimal Rust)
* Framework Frontend : Vue 3 (Composition API, `<script setup>`)
* Langage : TypeScript / HTML5 / CSS3
* Outil de Build : Vite
* Gestionnaire de paquets : npm
* Routage : Vue Router

Note sur TypeScript : Bien que l'enonce mentionne JavaScript, le projet utilise TypeScript pour assurer un typage strict des structures de donnees (Palette, Couleur) et eviter les erreurs de conversion au runtime.

<br><br>

# 2. INSTALLATION ET LANCEMENT


Prerequis :
* Node.js (version LTS recommandee)
* Rust & dependances systeme Tauri (necessaires pour executer le backend natif)

## Procedure :
#### 1. Installer les dependances frontend et backend :
   ```
   npm install
   ```

#### 2. Lancer l'application en mode developpement :
   ```
   npm run tauri dev
   ```

Une fenetre de bureau native s'ouvrira avec l'interface Vue.js.
<br><br><br>

# 3. FONCTIONNALITES DU TP2


## 1. Navigation a 3 Vues :
   * Accueil (AccueilView.vue) : Tableau de bord presentatif et statistiques/resume global des palettes.
   * Palettes (PalettesView.vue) : Affichage de la collection complete de palettes avec options de filtrage et de recherche.
   * Utilitaires (UtilitairesView.vue) : Outil interactif de conversion et selection de couleurs.

## 2. Gestion des Donnees Simulees (Couche Service) :
   * Les donnees sont isolees dans src/services/stockageDonnees.ts et initialisees depuis src/data/paletteExamples.ts.
   * Le service expose des mehtodes pour recuperer, ajouter, modifier, supprimer et filtrer les donnees sans charger directement les composants.

## 3. Formulaire avec Validation Frontend :
   * Formulaire complet dans AjoutPalette.vue permettant de creer une palette avec nom et selection de couleurs.
   * Validation avant envoi : verification du champ nom obligatoire (non vide) et du nombre minimal de couleurs requis.
   * Affichage dynamique des messages d'erreur ou de succes.

## 4. Interactions & Feedback Utilisateur :
   * Recherche et Filtres : Filtrage en temps reel des palettes par nom/mots-cles.
   * Actions multiples : Ajout, suppression avec modal de confirmation (confirmationSuppresion.vue), et modification de nuances.
   * Calculs simples & resumes : Calcul du nombre total de palettes, nombre de couleurs stockees et conversions automatiques (HEX / RGB / HSL).
   * Etats d'interface : Prise en charge des etats "liste vide", confirmation d'action et messages d'erreur/succes via MessageFooter.vue.
<br><br><br>

# 4. STRUCTURE DU PROJET


Pour le detail complet de l'architecture et l'arborescence des fichiers, veuillez consulter le fichier:
<br>  
[architecture-application.md](./architecture-application.md)

<br>

# 5. Auteurs


Mathieu Gosselin  
Clement Laflamme  
Francis Boisvert  
