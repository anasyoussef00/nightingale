<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { onMounted } from 'vue';

onMounted(() => {
  listen('playlist-selected', (event) => console.log(event.payload));
  listen('song-selected', (event) => console.log(event.payload));
});
</script>

<template>
  <main>
    <RouterView />
  </main>
</template>
