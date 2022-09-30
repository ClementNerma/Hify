<script lang="ts">
  import { usePageNavigator } from '../../navigation'

  const page = usePageNavigator()
</script>

<svelte:window />

<slot pageNav={page} />
