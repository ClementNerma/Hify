<script lang="ts">
import Modal from '../Modal/Modal.svelte'
import { currentError, errors, popError } from './ErrorDialog'
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
    <strong>
      {$currentError.title}

      {#if $errors.length === 2}
        (1 other error remaining)
      {:else if $errors.length > 2}
        ({$errors.length - 2} other errors remaining)
      {/if}
    </strong>

    <p>{$currentError.message}</p>

    <!-- TODO: $currentError.details -->
  </Modal>
{/if}
