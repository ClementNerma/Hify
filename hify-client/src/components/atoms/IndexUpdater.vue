<script setup lang="ts">
import Button from '@/components/atoms/Button.vue'
import { UpdateIndexDocument } from '@/graphql/generated/graphql'
import { useMutation } from '@urql/vue';
import { watch } from 'vue';

const emit = defineEmits<{
    updated: []
}>()

const { data, error, fetching, executeMutation } = useMutation(UpdateIndexDocument)

watch(data, () => emit('updated'))
</script>

<template>
    <Button @press="executeMutation({})" :disabled="fetching">
        Update the index (this might take a while)
    </Button>

    <span v-if="fetching">⌛</span>
    <span v-if="data">✅</span>
    <span v-if="error">❌ {{ error.message }}</span>
</template>
