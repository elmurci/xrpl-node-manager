import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
    path: "/404",
    name: "404",
    component: () => import("@/modules/404/index.vue"),
  },
  {
    path: "/",
    name: "home.dashboard",
    component: () => import("@/modules/Dashboard/index.vue"),
  },
  {
    path: "/configuration",
    name: "home.configuration",
    component: () => import("@/modules/Configuration/index.vue"),
  },
  {
    path: "/logs",
    name: "home.logs",
    component: () => import("@/modules/Logs/index.vue"),
  },
  {
    path: "/:catchAll(.*)",
    redirect: "/404",
  }
  ],
});

export default router;
