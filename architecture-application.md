# Structure de l'Architecture Actuelle (remise TP2)

## Stack Technique
- Framework Desktop : Tauri
- Framework Frontend : Vue.js 3
- Backend : Rust
- Build Tool : Vite
- Langage : TypeScript / HTML / CSS / Rust
- [README.md](./README.md)

## Structure des Fichiers

├── .gitignore
├── architecture-application.md
├── index.html
├── package-lock.json
├── package.json
├── README.md
│
├── src/
│   ├── assets/
│   ├── components/
│   │   ├── AjoutPalette.vue
│   │   ├── CarteCouleur.vue
│   │   ├── CartePalette.vue
│   │   ├── ChoisirPalette.vue
│   │   ├── ChoixCouleurs.vue
│   │   ├── MessageFooter.vue
│   │   ├── Navigation.vue
│   │   ├── OverlayColorPicker.vue
│   │   └── Slider.vue
│   ├── composables/
│   │   ├── useColorPicker.ts
│   │   ├── useCouleur.ts
│   │   ├── useInputMain.ts
│   │   ├── useMessage.ts
│   │   ├── usePalette.ts
│   │   └── useUtils.ts
│   ├── data/
│   │   └── paletteExamples.ts
│   ├── routeur/
│   │   └── index.ts
│   ├── types/
│   │   ├── couleur.ts
│   │   └── palette.ts
│   ├── views/
│   │   ├── AccueilView.vue
│   │   ├── PalettesView.vue
│   │   └── UtilitairesView.vue
│   ├── App.vue
│   ├── main.ts
│   ├── style.css
│   └── vite-env.d.ts
│
└── src-tauri/
    ├── capabilities/
    │   └── default.json
    ├── data/
    │   └── palettes_exemple.json
    ├── gen/
    │   └── schemas/
    ├── icons/
    ├── src/
    │   ├── palettes/               //palettes sert de dossier modeles
    │   │   ├── couleur.rs
    │   │   ├── mod.rs
    │   │   ├── palette.rs
    │   │   └── palettes.rs
    │   ├── color_pick.rs
    │   ├── commandes.rs
    │   ├── lib.rs
    │   ├── main.rs
    │   └── stockage.rs
    ├── build.rs
    ├── Cargo.lock
    ├── Cargo.toml
    └── tauri.conf.json