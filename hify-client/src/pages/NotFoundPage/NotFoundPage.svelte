<script lang="ts">
  import { useNavigate } from 'svelte-navigator'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  const navigate = useNavigate()
</script>

<SimpleNavigableItem onPress={() => navigate(-1)}>
  <h1>Page not found, press to go back.</h1>
</SimpleNavigableItem>

<style>
  h1 {
    display: inline-block;
    padding: 10px;
  }
</style>
