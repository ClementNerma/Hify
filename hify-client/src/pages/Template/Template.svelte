<script lang="ts">
  import { useNavigate } from 'svelte-navigator'
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'

  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'

  const navigate = useNavigate()
</script>

<NavigableWithHandlers onBack={() => navigate(-1)}>
  <NavigableList>
    <slot />
  </NavigableList>
</NavigableWithHandlers>
