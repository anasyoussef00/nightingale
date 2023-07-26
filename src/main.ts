import './tailwind.css';
import './assets/styles/main.css';

import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import router from './router';

import devtools from '@vue/devtools';

if (import.meta.env.DEV) devtools.connect('http://localhost', 8098);

createApp(App)
  .provide<boolean>('IS_DEV', import.meta.env.DEV)
  .use(router)
  .use(createPinia())
  .mount('#app');
