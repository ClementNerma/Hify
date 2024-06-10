<script setup lang="ts">
import { ref } from 'vue';
import Button, { type ButtonProps } from './Button.vue';

const model = defineModel<boolean>()
const props = defineProps<ButtonProps>()

const checkboxRef = ref<HTMLInputElement | null>(null)
</script>

<template>
    <Button v-bind="props" @press="model = !model">
        <input type="checkbox" v-model="model" ref="checkboxRef" />
        <slot />
    </Button>
</template>
