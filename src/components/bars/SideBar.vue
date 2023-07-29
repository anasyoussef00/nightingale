<script setup lang="ts">
import { ImgHTMLAttributes } from 'vue';
import SideBarRoutingBtn from '@/components/btns/SideBarRoutingBtn.vue';

defineProps<{ logo: ImgHTMLAttributes['src'] }>();
</script>

<template>
  <div
    class="flex flex-col gap-y-7 fixed inset-y-0 w-1/6 shadow-2xl px-3 py-4 bg-gradient-to-b from-violet-600 to-violet-500">
    <div class="flex flex-col gap-y-4">
      <div>
        <img :src="logo" alt="LOGO" />
      </div>

      <div class="flex flex-col gap-y-1">
        <h2 class="text-xl font-bold text-white">Browse</h2>
        <SideBarRoutingBtn :to="{ name: 'home' }" label="Home" icon="home" />
        <SideBarRoutingBtn label="Discover" icon="search" />
      </div>
    </div>

    <div class="flex flex-col gap-y-1">
      <h2 class="text-xl font-bold text-white">Your Library</h2>
      <SideBarRoutingBtn label="Liked Songs" icon="heart" />
      <SideBarRoutingBtn :to="{ name: 'home' }" label="Playlists" icon="book" />
      <SideBarRoutingBtn label="Recently Played" icon="clock" />
    </div>
  </div>
</template>
