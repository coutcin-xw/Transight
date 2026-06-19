import { createRouter, createWebHashHistory } from "vue-router";
import TranslationWindow from "./windows/TranslationWindow.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      name: "translate",
      component: TranslationWindow,
    },
    {
      path: "/settings",
      name: "settings",
      // 后续 M3 实现设置窗口组件
      component: () => import("./windows/SettingsWindow.vue"),
    },
  ],
});

export default router;
