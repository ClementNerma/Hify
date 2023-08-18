<script lang="ts">
  import { usePageNavigator } from '../../navigation'
  import InternalNavWrapper from '../InternalNavWrapper.svelte';

  const page = usePageNavigator()
</script>

<svelte:window />

<InternalNavWrapper navId={page.id}>
  <slot pageNav={page} />
</InternalNavWrapper>