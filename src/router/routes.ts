import { RouteRecordRaw } from 'vue-router';
import HomeView from '@/views/home/HomeView.vue';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
  },
];

export default routes;
