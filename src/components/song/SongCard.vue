<script setup lang="ts">
import useSongStore from '@/stores/song/song';
import { TSong } from '@/stores/song/types';
import { ImgHTMLAttributes } from 'vue';

withDefaults(
  defineProps<{
    imgSrc: ImgHTMLAttributes['src'];
    song: TSong;
  }>(),
  {
    imgSrc: '',
  },
);
</script>

<template>
  <div class="w-[300px] h-[300px] cursor-pointer group/cover">
    <div class="aspect-square shadow-lg rounded-xl relative flex justify-center items-center w-full h-full">
      <img :src="imgSrc" alt="KAS 7LO" class="rounded-xl" />
      <div
        class="absolute z-30 inset-0 opacity-0 group-hover/cover:opacity-20 bg-black transition-opacity duration-1000 ease-in-out rounded-xl">
      </div>
      <div
        class="absolute z-40 top-2 right-10 opacity-0 text-2xl group-hover/cover:opacity-100 transition-opacity duration-700 ease-in-out text-slate-100">
        <font-awesome-icon icon="heart" />
      </div>
      <div
        class="absolute z-40 top-2 right-2 opacity-0 text-2xl group-hover/cover:opacity-100 transition-opacity duration-700 ease-in-out text-slate-100">
        <font-awesome-icon icon="plus" />
      </div>
      <font-awesome-icon icon="circle-play"
        class="absolute z-40 opacity-0 text-6xl group-hover/cover:opacity-100 transition-opacity duration-700 ease-in-out text-slate-100" />
    </div>
    <div class="flex flex-col items-center">
      <p class="text-white text-xl font-bold">
        <span>{{ song.artist }} -</span> {{ song.title }}
      </p>
      <p class="text-slate-400 text-lg font-semibold">{{ song.album }}</p>
    </div>
  </div>
</template>
