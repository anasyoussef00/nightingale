<script setup lang="ts">
import { RouteLocationRaw, RouterLink } from 'vue-router';

withDefaults(
  defineProps<{ to?: RouteLocationRaw; label: string; icon: string }>(),
  {
    to: '#',
  },
);
</script>

<template>
  <RouterLink :to="to"
    class="inline-flex items-center gap-x-3 px-2.5 py-2 font-semibold hover:bg-violet-700 hover:text-white cursor-pointer text-slate-100 text-lg rounded-lg w-full"
    active-class="bg-violet-700">
    <font-awesome-icon :icon="`fa-solid fa-${icon}`" />
    {{ label }}
  </RouterLink>
</template>
