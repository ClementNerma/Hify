<script setup lang="ts">
import type { NavigableGridProps } from '@/navigable/headless/NavigableGrid/NavigableGrid'
import NavigableGrid from '@/navigable/headless/NavigableGrid/NavigableGrid.vue'
import { computed } from 'vue';

const props = defineProps<NavigableGridProps>()
</script>

<template>
  <div class="container grid text-center min-w-full">
    <NavigableGrid v-bind="props">
      <slot />
    </NavigableGrid>
  </div>
</template>

<style scoped>
.container {
  grid-template-columns: repeat(v-bind(columns), 1fr);
}
</style>
