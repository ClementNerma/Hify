<script setup lang="ts">
import Modal from '../molecules/Modal.vue'
import { currentError, errors, popError } from './ErrorDialog'
</script>

<template>
    <Modal v-if="currentError" open :buttons="[{
        label: 'Close', onPress: () => popError(),
    }]">
        <strong>
            {{ currentError.title }}

            <span v-if="errors.length === 2">
                (1 other error remaining)
            </span>

            <span v-else-if="errors.length > 2">
                ({{ errors.length - 1 }} other errors remaining)
            </span>
        </strong>

        <p>{{ currentError.message }}</p>

        <p>{{ currentError.details }}</p>
    </Modal>

</template>