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

<style>
  :global(navigable-item-wrapper :first-child) {
    border: 3px solid transparent;
  }

  :global(navigable-item-wrapper.focused :first-child) {
    background-color: pink;
    border: 3px solid pink;
    border-radius: 5px;
  }
</style>
