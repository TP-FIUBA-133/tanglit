<script setup lang="ts">
defineProps<{
  html: string;
}>();

const availableThemes = ["pico", "water", "sakura", "latex"];

const html_theme = defineModel("html_theme");
</script>

<template>
  <div class="html-preview">
    <div class="theme-selectors">
      <span class="preview-title">Document</span>
      <div class="theme-selector">
        <label for="theme-select">Doc theme: </label>
        <select id="theme-select" v-model="html_theme">
          <option v-for="theme in availableThemes" :key="theme" :value="theme">
            {{ theme }}
          </option>
        </select>
      </div>
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
}

.theme-selectors {
  display: flex;
  gap: 20px;
  margin: 5px;
  align-items: center;
}

.preview-title {
  font-size: 14pt;
  font-weight: bold;
  color: #595959;
  margin-right: 10px;
}
</style>
