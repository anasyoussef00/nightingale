import { RouteRecordRaw } from 'vue-router';
import HomeView from '@/views/home/HomeView.vue';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
  },
  {
    path: '/songs',
    name: 'songs',
    children: [
      {
        path: 'recently-added-songs',
        name: 'recently-added-songs',
        component: () => import('@/views/songs/RecentlyAddedSongsView.vue'),
      },
    ],
  },
];

export default routes;
