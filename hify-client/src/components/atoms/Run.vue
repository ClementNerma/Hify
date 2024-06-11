<script setup lang="ts">
import { onBeforeUpdate, onMounted } from 'vue';

const emit = defineEmits<{
    run: []
}>()

onMounted(() => emit('run'))
onBeforeUpdate(() => emit('run'))
</script>

<template></template>