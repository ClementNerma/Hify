<script setup lang="ts">
import NavigableRow from '@/navigable/headless/NavigableRow/NavigableRow.vue'
import type { NavigableRowProps } from '@/navigable/headless/NavigableRow/NavigableRow'

const props = defineProps<NavigableRowProps>()
</script>

<template>
  <div class="row">
    <NavigableRow v-bind="props">
      <slot />
    </NavigableRow>
  </div>
</template>

<style scoped>
.row {
  overflow-x: hidden;
  display: flex;
  flex-direction: row;
}
</style>