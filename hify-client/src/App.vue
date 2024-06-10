<script setup lang="ts">
import { RouterView } from 'vue-router'
import Layout from './views/Layout.vue'
import { provideClient } from '@urql/vue';
import { gqlClient } from './global/urql-client';

provideClient(gqlClient)
</script>

<template>
  <Layout>
    <RouterView />
  </Layout>
</template>
