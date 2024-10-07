<script setup lang="ts">
import { ref } from 'vue';
import Button, { type ButtonProps } from './Button.vue';
import Emoji from './Emoji.vue';

const model = defineModel<boolean>()
const props = defineProps<ButtonProps>()

const checkboxRef = ref<HTMLInputElement | null>(null)
</script>

<template>
    <Button v-bind="props" @press="model = !model">
        <span>
            <Emoji>
                <template v-if="model">☑️</template>
                <template v-else>🔲</template>
            </Emoji>
        </span>

        <slot />
    </Button>
</template>
