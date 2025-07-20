<script setup lang="ts">
import { provideClient } from '@urql/vue'
import { RouterView } from 'vue-router'
import { gqlClient } from './global/urql-client'
import Layout from './Layout.vue'

provideClient(gqlClient)
</script>

<template>
  <Layout>
    <RouterView />
  </Layout>
</template>
