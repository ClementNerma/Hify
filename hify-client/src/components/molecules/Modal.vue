<script lang="ts">
export type ModalButton = {
  label: string
  onPress: () => void | boolean | Promise<void | boolean>
}
</script>

<script setup lang="ts">
import Row from '@/navigable/ui/molecules/Row/Row.vue'
import Button from '../atoms/Button.vue'
import { NavigableItem, type RequestFocus, getParentNavigable } from '@/navigable/navigation'
import NavigableList from '@/navigable/headless/NavigableList/NavigableList.vue'
import { onMounted, onUpdated, ref } from 'vue'
import Run from '../atoms/Run.vue'

const props = defineProps<{
  open?: boolean,
  buttons: ModalButton[],
  onOpen?: () => void
}>()

const open = ref(props.open)
const wasOpen = ref(false)
const loading = ref(false)
const prevFocusItem = ref<NavigableItem<unknown> | null>(null)
const buttonsRequestFocus = ref<RequestFocus[]>(new Array(props.buttons.length))

const nav = getParentNavigable()

async function onButtonPress(button: ModalButton) {
  if (loading.value) {
    return
  }

  loading.value = true

  if ((await button.onPress()) !== false) {
    open.value = false
  }

  loading.value = false
}

async function focusButtonOnOpen() {
  if (wasOpen.value !== open.value) {
    wasOpen.value = open.value

    if (open.value) {
      prevFocusItem.value = nav.page.focusedItem()
      buttonsRequestFocus.value[0]?.()
      props.onOpen?.()
    } else {
      prevFocusItem.value?.requestFocus()
    }
  }
}

onMounted(focusButtonOnOpen)
onUpdated(focusButtonOnOpen)
</script>

<template>
  <div class="fixed inset-0 bg-black/50 z-10" v-if="open">
    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 p-2.5 rounded bg-gray-200 text-black">
      <NavigableList trapped>
        <slot />

        <div class="mt-4">
          <Row>
            <Button v-for="button, i in buttons" :key="button.label" v-slot="{ item, focused }"
              @press="onButtonPress(button)">

              <Run @run="buttonsRequestFocus[i] = () => item.requestFocus()" />

              <em v-if="loading && focused">Loading...</em>
              <span v-else>{{ button.label }}</span>
            </Button>

          </Row>
        </div>
      </NavigableList>
    </div>
  </div>
</template>
