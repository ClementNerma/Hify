<script setup lang="ts">
import { getAlbumArtUrl } from '@/global/constants';
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import { computed } from 'vue';

const props = defineProps<{
  track: AudioTrackFragment | null,
  dim: boolean
}>()

const album = computed(() => props.track?.metadata.tags.album)
const background = computed(() => album.value?.hasArt ? `url("${getAlbumArtUrl(album.value)}")` : 'transparent')
const backdropFilter = computed(() => `blur(20px) brightness(${props.dim ? 0.3 : 0.4})`)
</script>

<template>
  <div class="background fixed inset-0 -z-20 bg-center bg-no-repeat bg-cover" :style="`--background: ${background}`" />
  <div class="filter fixed inset-0 -z-10" :style="`--backdrop-filter: ${backdropFilter}`" />
</template>

<style scoped>
.background {
  background: var(--background);
}

.filter {
  backdrop-filter: var(--backdrop-filter);
}
</style>
