<script setup lang="ts">
import '@fortawesome/fontawesome-free/css/all.min.css'
import MessageFooter from "./components/MessageFooter.vue";
import Navigation from "./components/Navigation.vue";
</script>

<template>
  <div class="flex flex-col h-full">

    <Navigation />

    <!-- Zone centrale flexible -->
    <main class="flex flex-col flex-1 min-h-0">
      <RouterView />
    </main>

    <MessageFooter/>

  </div>
</template>

