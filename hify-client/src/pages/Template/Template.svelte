<script lang="ts">
  import { useNavigate } from 'svelte-navigator'
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigablePage from '../../navigable/NavigablePage/NavigablePage.svelte'

  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'

  const navigate = useNavigate()
</script>

<NavigablePage>
  <NavigableWithHandlers onBack={() => navigate(-1)}>
    <NavigableList>
      <slot />
    </NavigableList>
  </NavigableWithHandlers>
</NavigablePage>

<style>
  :global(navigable-item-wrapper :first-child) {
    border: 3px solid transparent;
    cursor: pointer;
  }

  :global(navigable-item-wrapper.focused :first-child, navigable-item-wrapper:hover :first-child) {
    background-color: pink;
    border: 3px solid pink;
    border-radius: 5px;
  }
</style>
