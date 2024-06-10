<script setup lang="ts">
import { getArtUri } from '@/global/constants'
import type { ArtFragment } from '@/graphql/generated/graphql';

defineProps<{ art: ArtFragment | null | undefined }>()

defineSlots<{
  default(props: { src: string }): unknown
}>()
</script>

<template>
  <slot :src="art ? getArtUri(art.id) : 'about:blank'" />
</template>