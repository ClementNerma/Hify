<script lang="ts">
  import { useNavigate } from 'svelte-navigator'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  const navigate = useNavigate()

  let focused = false
</script>

<SimpleNavigableItem
  onPress={() => navigate(-1)}
  onFocusChange={(has) => {
    focused = has
  }}
>
  <h1 class={focused ? 'focused' : ''}>Page not found, press to go back.</h1>
</SimpleNavigableItem>

<style>
  h1 {
    display: inline-block;
    padding: 10px;
  }

  .focused {
    background: pink;
  }
</style>
