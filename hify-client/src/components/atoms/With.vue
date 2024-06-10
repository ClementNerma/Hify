<script setup lang="ts" generic="T">
const { data } = defineProps<{
  data: T
}>()

defineSlots<{
  default(props: { data: T }): unknown
}>()
</script>

<template>
  <slot :data="data as any /* HACK: required because of Vue's typing */" />
</template>
