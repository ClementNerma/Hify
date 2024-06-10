<script setup lang="ts">
</script>

<template>
    <div class="centered">
        <slot />
    </div>
</template>

<style scoped>
.centered {
    display: flex;
    flex-direction: column;
    align-items: center;
}
</style>