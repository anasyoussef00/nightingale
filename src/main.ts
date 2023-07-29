import './tailwind.css';
import './assets/styles/main.css';

import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import router from './router';

/* import the fontawesome core */
import { library } from '@fortawesome/fontawesome-svg-core';

/* import font awesome icon component */
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';

import * as icons from './icons.ts';

library.add(icons);

import devtools from '@vue/devtools';

if (import.meta.env.DEV) devtools.connect('http://localhost', 8098);

createApp(App)
  .provide<boolean>('IS_DEV', import.meta.env.DEV)
  .use(router)
  .use(createPinia())
  .component('font-awesome-icon', FontAwesomeIcon)
  .mount('#app');
