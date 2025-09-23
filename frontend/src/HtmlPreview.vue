<script setup lang="ts">
defineProps<{
  html: string;
}>();
</script>

<template>
  <iframe :srcdoc="html"></iframe>
</template>

<style scoped>
iframe {
  width: 100%;
  height: 100%;
  border: none;
}
</style>
