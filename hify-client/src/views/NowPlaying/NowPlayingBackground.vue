<script setup lang="ts">
import { computed } from 'vue'
import { getAlbumArtUrl } from '@/global/constants'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'

const props = defineProps<{
	track: AudioTrackFragment | null
	dim: boolean
}>()

const album = computed(() => props.track?.metadata.tags.album)
const background = computed(() =>
	album.value?.hasArt ? `url("${getAlbumArtUrl(album.value, 'large')}")` : 'transparent',
)
const backdropFilter = computed(() => `blur(20px) brightness(${props.dim ? 0.3 : 0.4})`)
</script>

<template>
  <div class="background fixed inset-0 -z-20 bg-center bg-no-repeat bg-cover" />
  <div class="filter fixed inset-0 -z-10" />
</template>

<style scoped>
.background {
  background-image: v-bind(background);
}

.filter {
  backdrop-filter: v-bind(backdropFilter);
}
</style>
