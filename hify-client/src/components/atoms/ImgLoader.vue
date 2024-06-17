<script setup lang="ts">
import { getArtUri } from '@/global/constants'

defineProps<{ type: 'album' | 'artist', item: { id: string, hasArt: boolean } }>()

defineSlots<{
  default(props: { src: string }): unknown
}>()
</script>

<template>
  <slot :src="item.hasArt ? getArtUri(type, item.id) : 'about:blank'" />
</template>