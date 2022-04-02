<script lang="ts">
  import { useNavigate } from 'svelte-navigator'

  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'

  const navigate = useNavigate()
</script>

<NavigableWithHandlers onBack={() => navigate(-1)}>
  <slot />
</NavigableWithHandlers>
