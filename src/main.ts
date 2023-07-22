import './tailwind.css';
import './assets/styles/main.css';

import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import router from './router';

createApp(App)
  .provide<boolean>('IS_DEV', import.meta.env.DEV)
  .use(router)
  .use(createPinia())
  .mount('#app');
