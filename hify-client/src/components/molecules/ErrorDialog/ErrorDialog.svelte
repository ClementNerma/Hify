<script lang="ts">
  import Modal from '../Modal/Modal.svelte'
  import { currentError, popError } from './ErrorDialog'
</script>

{#if $currentError}
  <Modal
    open
    buttons={[
      {
        label: 'Close',
        onPress: () => popError(),
      },
    ]}
  >
    <strong>{$currentError.title}</strong>
    <p>{$currentError.message}</p>
    <!-- TODO: $currentError.details -->
  </Modal>
{/if}
