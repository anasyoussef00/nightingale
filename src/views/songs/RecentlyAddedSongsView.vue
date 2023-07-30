<script setup lang="ts">
import SongCard from '@/components/song/SongCard.vue';
import useSongStore from '@/stores/song/song';
import { TSong } from '@/stores/song/types';
import { onMounted, ref } from 'vue';

const songStore = useSongStore();

const songs = ref<TSong[]>();

onMounted(async () => (songs.value = await songStore.fetchAddedSongs()));

const setCurrentSong = (song: TSong) => (songStore.currentSong = song);
</script>

<template>
  <section class="min-h-screen p-4">
    <div class="grid grid-cols-4 gap-4">
      <SongCard v-for="(song, index) in songs" :key="index" :img-src="song.cover" :song="song" class="col-span-1"
        @click="setCurrentSong(song)" />
    </div>
  </section>
</template>
