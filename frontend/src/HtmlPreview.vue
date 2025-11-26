<script setup lang="ts">
defineProps<{
  html: string;
}>();

const availableThemes = ["pico", "water", "sakura", "latex"];

const html_theme = defineModel("html_theme");
</script>

<template>
  <div class="html-preview">
    <div class="theme-selector">
      <label for="theme-select">Doc theme: </label>
      <select id="theme-select" v-model="html_theme">
        <option v-for="theme in availableThemes" :key="theme" :value="theme">
          {{ theme }}
        </option>
      </select>
    </div>
    <iframe :srcdoc="html"></iframe>
  </div>
</template>

<style scoped>
.html-preview {
  height: 100%;
  display: flex;
  flex-direction: column;
}

iframe {
  width: 100%;
  height: 100%;
  border: none;
}
</style>
