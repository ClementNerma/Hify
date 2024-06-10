<script setup lang="ts">
import { onMounted, ref } from 'vue';

const { delayMillis = 1000 } = defineProps<{ delayMillis?: number }>()

const visible = ref(false)

onMounted(() => {
    setTimeout(() => {
        visible.value = true
    }, delayMillis)
})
</script>

<template>
    <h2 v-if="visible">Still loading...</h2>
</template>
