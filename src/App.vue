<script setup lang="ts">
import { Event, UnlistenFn, listen } from '@tauri-apps/api/event';
import { onMounted, ref } from 'vue';
import SideBar from '@/components/bars/SideBar.vue';
import PlayerController from '@/components/controls/PlayerController.vue';
import { TSong } from '@/stores/song/types';
import useSongStore from '@/stores/song/song';

const songStore = useSongStore();

const unlisten = ref<Promise<UnlistenFn>>();

onMounted(() => {
  unlisten.value = listen('playlist-selected', (event) =>
    console.log(event.payload),
  );
  unlisten.value = listen('song-selected', (event: Event<TSong>) =>
    songStore.songsList.push(event.payload),
  );

  songStore.fetchAddedSongs();
});
</script>

<template>
  <div class="grid grid-cols-12 relative">
    <aside class="self-start sticky col-span-2 z-30">
      <SideBar logo="/src/assets/vue.svg" />
    </aside>

    <main class="col-span-10 bg-gradient-to-br from-violet-700 via-violet-800 to-violet-900 z-20">
      <RouterView />
    </main>

    <footer class="absolute inset-x-0 bottom-0 z-40 w-full">
      <PlayerController :song="songStore.currentSong" />
    </footer>
  </div>
</template>
