import Accueil from "../views/Accueil.vue";
import AccueilView from "../views/AccueilView.vue";
import Page1 from "../views/Page1.vue";
import Page2 from "../views/Page2.vue";
import { createRouter, createWebHashHistory } from "vue-router";
import PalettesView from "../views/PalettesView.vue";
import UtilitairesView from "../views/UtilitairesView.vue";

const routes = [
  {
    path: "/",
    name: "accueil",
    component: AccueilView,
  },
  {
    path: "/palettes",
    name: "palettes",
    component: PalettesView,
  },
  {
    path: "/utilitaires",
    name: "utilitaires",
    component: UtilitairesView,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
  scrollBehavior() {
    return { top: 0 };
  },
});

export default router;
