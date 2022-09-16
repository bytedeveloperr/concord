import { createRouter, createWebHistory } from "vue-router";
import Connect from "./views/Connect.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [{ path: "/connect", name: "connect", component: Connect }],
});
