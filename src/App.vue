<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { onMounted } from 'vue';
import SideBar from '@/components/bars/SideBar.vue';

onMounted(() => {
  listen('playlist-selected', (event) => console.log(event.payload));
  listen('song-selected', (event) => console.log(event.payload));
});
</script>

<template>
  <div class="grid grid-cols-12 gap-x-14 w-full">
    <aside class="self-start sticky inset-y-0 col-span-2 overflow-y-scroll w-full z-30">
      <SideBar logo="/src/assets/vue.svg" />
    </aside>

    <main class="col-span-10">
      <RouterView />
    </main>
  </div>
</template>
