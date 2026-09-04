<script setup lang="ts">
import '@fortawesome/fontawesome-free/css/all.min.css'
import MessageFooter from "./components/MessageFooter.vue";
import Navigation from "./components/Navigation.vue";
</script>

<template>
  <div class="flex flex-1 flex-col">
    <Navigation />
    <main class="flex flex-1 flex-col">
      <RouterView />
    </main>
    <MessageFooter/>
  </div>
</template>
