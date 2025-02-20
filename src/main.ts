import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import "primeicons/primeicons.css";
import { createPinia } from "pinia";
import "./assets/main.css";
import { DataCanvasPreset } from "./utils/theme";

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(PrimeVue, {
  theme: {
    preset: DataCanvasPreset,
  },
});
app.mount("#app");
