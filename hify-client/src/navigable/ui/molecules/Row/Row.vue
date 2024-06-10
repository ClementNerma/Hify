<script setup lang="ts">
import NavigableRow from '@/navigable/headless/NavigableRow/NavigableRow.vue'
import type { NavigableRowProps } from '@/navigable/headless/NavigableRow/NavigableRow'

const props = defineProps<NavigableRowProps>()
</script>

<template>
  <div class="flex flex-row overflow-x-hidden">
    <NavigableRow v-bind="props">
      <slot />
    </NavigableRow>
  </div>
</template>
