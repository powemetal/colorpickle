import AccueilView from "../views/AccueilView.vue";
import { createRouter, createWebHashHistory } from "vue-router";
import PalettesView from "../views/PalettesView.vue";
import UtilitairesView from "../views/UtilitairesView.vue";
import OverlayColorPicker from "../components/OverlayColorPicker.vue";

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
  {
      path: '/overlay',
      name: 'overlay',
      component: OverlayColorPicker,
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
