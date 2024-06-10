<script setup lang="ts">
import { usePageNavigator } from '../../navigation'
import InternalNavWrapper from '../InternalNavWrapper.vue'

const page = usePageNavigator()
</script>

<svelte:window />

<template>
  <InternalNavWrapper :nav="page" :props="{}">
    <slot :page />
  </InternalNavWrapper>
</template>
