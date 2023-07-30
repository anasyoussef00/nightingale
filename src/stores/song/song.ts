import { defineStore } from 'pinia';
import { ref } from 'vue';
import { TSong } from './types';
import { invoke } from '@tauri-apps/api';

const useSongStore = defineStore('song-store', () => {
  const songsList = ref<TSong[]>([]);
  const currentSong = ref<TSong>();

  const fetchAddedSongs = async (): Promise<TSong[]> => {
    try {
      const songs = await invoke<TSong[]>('fetch_added_songs');
      songsList.value = songs;
      return songs;
    } catch (err) {
      return songsList.value;
    }
  };

  const toggleSongLike = async (index: number) => {
    try {
      await invoke('toggle_song_like', { song: songsList.value[index] });
      songsList.value[index].liked = !songsList.value[index].liked;
    } catch (err) {
      console.error(err);
    }
  };

  return {
    currentSong,
    fetchAddedSongs,
    toggleSongLike,
    songsList,
  };
});

export default useSongStore;
