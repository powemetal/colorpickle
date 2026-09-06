# Structure de l'Architecture Actuelle (remise TP2)

## Stack Technique
- Framework Desktop : Tauri
- Framework Frontend : Vue.js 3 (Composition API)
- Build Tool : Vite
- Langage : TypeScript / HTML / CSS

## Structure des Fichiers

├── index.html  
├── vite.config.ts  
├── package.json  
└── src/  
    ├── main.ts  
    ├── App.vue  
    ├── style.css  
    ├── routeur/  
    │   └── index.ts  
    ├── services/  
    │   └── donneesService.ts  
    ├── data/  
    │   └── paletteExamples.ts  
    ├── types/  
    │   ├── couleur.ts  
    │   └── palette.ts  
    ├── composables/  
    │   ├── usePalette.ts  
    │   ├── useCouleur.ts  
    │   ├── useColorPicker.ts  
    │   ├── useInputMain.ts  
    │   ├── useMessage.ts  
    │   └── useUtils.ts  
    ├── components/  
    │   ├── AjoutPalette.vue  
    │   ├── CarteCouleur.vue  
    │   ├── CartePalette.vue  
    │   ├── ChoisirPalette.vue  
    │   ├── ChoixCouleurs.vue  
    │   ├── Navigation.vue  
    │   ├── OverlayColorPicker.vue  
    │   ├── Slider.vue  
    │   └── MessageFooter.vue  
    └── views/  
        ├── AccueilView.vue  
        ├── PalettesView.vue  
        └── UtilitairesView.vue