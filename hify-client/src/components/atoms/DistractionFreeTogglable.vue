<script setup lang="ts">
import { distractionFreeMode } from '@/global/stores/distraction-free'
</script>

<template>
    <div :class="{ hidden: distractionFreeMode }">
        <slot />
    </div>
</template>

<style scoped>
div:not(.hidden) {
    transition: all 0.5s;
}

.hidden {
    transition: all 0.7s;
    opacity: 0;
}
</style>
