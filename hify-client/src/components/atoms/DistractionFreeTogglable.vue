<script setup lang="ts">
import { distractionFreeMode } from '@/global/stores/distraction-free'
</script>

<template>
    <div class="transition" :class="distractionFreeMode ? ['opacity-0', 'duration-700'] : ['duration-500']">
        <slot />
    </div>
</template>
