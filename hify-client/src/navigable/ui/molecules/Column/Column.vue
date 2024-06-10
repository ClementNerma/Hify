<script setup lang="ts">
import type { NavigableList as NavigableListClass, NavigableListProps } from '@/navigable/headless/NavigableList/NavigableList'
import NavigableList from '@/navigable/headless/NavigableList/NavigableList.vue'

defineProps<NavigableListProps>()

defineSlots<{
  default(props: { column: NavigableListClass }): unknown
}>()
</script>

<template>
  <div class="flex flex-col overflow-x-auto">
    <NavigableList v-slot="{ list }">
      <slot :column="list" />
    </NavigableList>
  </div>
</template>
