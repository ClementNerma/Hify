<script setup lang="ts">
import Button from '@/components/atoms/Button.vue'
import { UpdateIndexDocument } from '@/graphql/generated/graphql'
import { useMutation } from '@urql/vue'
import { watch } from 'vue'
import Emoji from './Emoji.vue'

const emit = defineEmits<{
	updated: []
}>()

const { data, error, fetching, executeMutation } = useMutation(UpdateIndexDocument)

watch(data, () => emit('updated'))
</script>

<template>
    <Button @press="executeMutation({})" :disabled="fetching">
        Update the index (this might take a while)

        <Emoji v-if="fetching">⌛</Emoji>
        <Emoji v-else-if="data">✅</Emoji>
        <Emoji v-else-if="error">❌ {{ error.message }}</Emoji>
    </Button>
</template>
