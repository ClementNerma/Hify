<script setup lang="ts">
import { getArtUri } from '@/global/constants';
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import { computed } from 'vue';

const props = defineProps<{
  track: AudioTrackFragment | null,
  dim: boolean
}>()

const art = computed(() => props.track?.metadata.tags.album?.art)
const background = computed(() => art.value ? `url("${getArtUri(art.value.id)}")` : 'transparent')
const backdropFilter = computed(() => `blur(20px) brightness(${props.dim ? 0.3 : 0.4})`)
</script>

<template>
  <div class="background" :style="`--background: ${background}`" />
  <div class="filter" :style="`--backdrop-filter: ${backdropFilter}`" />
</template>

<style scoped>
.background {
  position: fixed;

  top: 0;
  left: 0;
  right: 0;
  bottom: 0;

  z-index: -2;

  background: var(--background);
  background-position: center;
  background-repeat: no-repeat;
  background-size: cover;
}

.filter {
  position: fixed;

  top: 0;
  left: 0;
  right: 0;
  bottom: 0;

  z-index: -1;

  backdrop-filter: var(--backdrop-filter);
}
</style>
