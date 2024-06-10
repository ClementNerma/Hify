<script setup lang="ts">
import type { NavigableGridProps } from '@/navigable/headless/NavigableGrid/NavigableGrid'
import NavigableGrid from '@/navigable/headless/NavigableGrid/NavigableGrid.vue'
import { computed } from 'vue';

const props = defineProps<NavigableGridProps>()

const columnsSize = computed(() => new Array(props.columns).fill(`${100 / props.columns}%`).join(' '))
</script>

<template>
  <div class="container grid text-center" :style="`--columns-size: ${columnsSize}`">
    <NavigableGrid v-bind="props">
      <slot />
    </NavigableGrid>
  </div>
</template>

<style scoped>
.container {
  grid-template-columns: var(--columns-size);
}
</style>
