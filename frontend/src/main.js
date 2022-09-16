import App from "./App.vue";
import { createApp } from "vue";
import { pinia } from "./plugins/pinia";
import { vuetify } from "./plugins/vuetify";
import { loadFonts } from "./plugins/webfontloader";

import "./main.css";
import { router } from "./router";

loadFonts();
createApp(App).use(router).use(pinia).use(vuetify).mount("#app");
