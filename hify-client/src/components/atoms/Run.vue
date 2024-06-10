<script setup lang="ts">
import { onBeforeUpdate, onMounted } from 'vue';

// TODO: use an event instead
const { run } = defineProps<{
    run: () => unknown
}>()

onMounted(run)
onBeforeUpdate(run)
</script>

<template></template>