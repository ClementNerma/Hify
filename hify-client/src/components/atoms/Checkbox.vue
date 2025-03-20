<script setup lang="ts">
import Button, { type ButtonProps } from './Button.vue'
import Emoji from './Emoji.vue'

const model = defineModel<boolean>()
const props = defineProps<ButtonProps>()
</script>

<template>
    <Button v-bind="props" @press="model = !model">
        <span>
            <Emoji>
                {{ model ? '☑️' : '🔲' }}
            </Emoji>
        </span>

        <slot />
    </Button>
</template>
