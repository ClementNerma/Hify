<script setup lang="ts">
import { onMounted, ref } from 'vue';

const { delayMillis = 1000 } = defineProps<{ delayMillis?: number, error: string | null | undefined }>()

const visible = ref(false)

onMounted(() => {
    setTimeout(() => {
        visible.value = true
    }, delayMillis)
})
</script>

<template>
    <h2 v-if="visible">
        <span v-if="!error">Still loading...</span>
        <span v-else class="text-red-600 font-bold">{{ error }}</span>
    </h2>
</template>
