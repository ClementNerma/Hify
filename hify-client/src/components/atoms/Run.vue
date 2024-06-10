<script setup lang="ts">
import { onMounted, onUpdated } from 'vue';

// TODO: use an event instead
const { run } = defineProps<{
    run: () => unknown
}>()

onMounted(() => {
    run()
})

onUpdated(() => {
    run()
})
</script>

<template></template>