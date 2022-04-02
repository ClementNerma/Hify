<script lang="ts">
  import { setContext } from 'svelte'

  import { getContext } from 'svelte'
  import { asNavigator, NAVIGATION_CTX, useNavigator } from '../handlers/nav'

  const ctx = asNavigator(getContext(NAVIGATION_CTX))

  const nav = useNavigator(ctx)

  setContext(NAVIGATION_CTX, nav)

  ctx.register({ type: 'row', nav })
</script>

<slot />
