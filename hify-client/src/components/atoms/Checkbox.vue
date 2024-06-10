<script setup lang="ts">
import { ref, watch } from 'vue';
import Button from './Button.vue';
import type { SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';

const model = defineModel<boolean>()

const props = defineProps<SimpleNavigableItemProps>()

const checkboxRef = ref<HTMLInputElement | null>(null)
</script>

<template>
    <Button v-bind="props" @press="model = !model">
        <input type="checkbox" v-model="model" ref="checkboxRef" />
        <span>
            <slot />
        </span>
    </Button>
</template>
