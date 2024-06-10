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
import { onUpdated, ref } from 'vue'

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

onUpdated(() => {
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
})
</script>

<template>
  <div class="modal" v-if="open">
    <div class="modal-inner">
      <NavigableList trapped>
        <slot />

        <div class="buttons">
          <Row>
            <Button v-for="button, i in buttons" :key="button.label" v-slot="{ item, focused }"
              @press="onButtonPress(button)">
              <!-- TODO: bind button to nth value -->
              <em v-if="loading && focused">Loading...</em>
              <span v-else>{{ button.label }}</span>
            </Button>

          </Row>
        </div>
      </NavigableList>
    </div>
  </div>
</template>

<style scoped>
.modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;

  background-color: rgba(0, 0, 0, 0.5);
}

.modal-inner {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);

  padding: 10px;
  font-size: 15px;
  border-radius: 5px;
  background-color: lightgray;
  color: black;
}

.buttons {
  margin-top: 15px;
}
</style>
