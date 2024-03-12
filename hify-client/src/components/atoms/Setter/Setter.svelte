<script lang="ts" context="module">
import { type Writable, writable } from 'svelte/store'

export function setterStore<T>(): Writable<T | null> {
	return writable(null)
}
</script>

<script lang="ts" generics="T">
  import { beforeUpdate } from 'svelte'

  export let value: T
  export let writeTo: Writable<T | null>

  beforeUpdate(() => {
    writeTo.set(value)
  })
</script>
