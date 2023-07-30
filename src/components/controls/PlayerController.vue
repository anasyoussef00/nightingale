<script setup lang="ts">
import { TSong } from '@/stores/song/types';
import { ref } from 'vue';
defineProps<{ song?: TSong }>();

const volume = ref<number>(100);
</script>

<template>
  <div class="flex justify-between items-center w-full border-t-2 border-t-violet-500 shadow-xl bg-white px-3">
    <div v-if="song" class="flex items-center py-3">
      <div class="w-1/4 rounded-lg pr-3">
        <img :src="song.cover" alt="COVER" class="rounded-lg" />
      </div>
      <div class="flex flex-col mr-5">
        <p class="font-bold text-xl">
          {{ song.title }} - <span>[{{ song.album }}]</span>
        </p>
        <p class="text-slate-500 text-lg">{{ song.artist }}</p>
      </div>
      <font-awesome-icon icon="heart" class="text-2xl text-violet-500" />
    </div>

    <div class="flex flex-col justify-center items-center gap-y-2 w-full">
      <div class="flex items-center gap-x-4">
        <font-awesome-icon icon="shuffle" />
        <font-awesome-icon icon="backward" class="text-2xl" />

        <button class="flex justify-center items-center rounded-full bg-violet-500 px-2.5 py-2">
          <font-awesome-icon icon="circle-play" class="text-white w-6 h-6" />
        </button>

        <font-awesome-icon icon="forward" class="text-2xl" />
        <font-awesome-icon icon="repeat" />
      </div>
      <div class="inline-flex justify-center items-center gap-x-2 w-full">
        <span class="text-slate-500">0:00</span>
        <input type="range" class="w-1/2" />
        <span class="text-slate-500">5:13</span>
      </div>
    </div>

    <div class="flex items-center gap-x-3">
      <font-awesome-icon icon="volume-high" />
      <input v-model.number="volume" type="range" min="0" max="100" step="1" />
    </div>
  </div>
</template>
