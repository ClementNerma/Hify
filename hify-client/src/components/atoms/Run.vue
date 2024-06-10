<script setup lang="ts">
import { onMounted, onUpdated } from 'vue';

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