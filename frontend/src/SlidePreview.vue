<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

const { slides_html } = defineProps<{ slides_html: string }>();

const AVAILABLE_MAIN_THEMES = [
  "black",
  "white",
  "league",
  "beige",
  "sky",
  "night",
  "serif",
  "simple",
  "solarized",
  "blood",
  "moon",
  "dracula",
  "black-contrast",
  "white-contrast",
];

const AVAILABLE_CODE_THEMES = [
  "agate",
  "ascetic",
  "dark",
  "default",
  "github",
  "github-dark",
  "github-dark-dimmed",
  "monokai",
  "obsidian",
];

const main_theme = defineModel("main_theme");
const code_theme = defineModel("code_theme");

const isFullscreen = ref(false);

function onMessage(event: MessageEvent) {
  if (event.data?.type === "toggle-fullscreen") {
    isFullscreen.value = !isFullscreen.value;
  }
}

onMounted(() => window.addEventListener("message", onMessage));
onUnmounted(() => window.removeEventListener("message", onMessage));
</script>

<template>
  <div v-if="!isFullscreen" class="theme-selectors">
    <span class="preview-title">Slides</span>
    <div class="theme-selector">
      <label for="theme-select">Main theme: </label>
      <select id="theme-select" v-model="main_theme">
        <option v-for="theme in AVAILABLE_MAIN_THEMES" :key="theme" :value="theme">
          {{ theme }}
        </option>
      </select>
    </div>

    <div class="code-theme-selector">
      <label for="code-theme-select">Code theme: </label>
      <select id="code-theme-select" v-model="code_theme">
        <option v-for="theme in AVAILABLE_CODE_THEMES" :key="theme" :value="theme">
          {{ theme }}
        </option>
      </select>
    </div>
  </div>
  <iframe :srcdoc="slides_html" :class="{ fullscreen: isFullscreen }" allowfullscreen></iframe>
</template>

<style scoped>
iframe {
  height: 100%;
  width: 100%;
}

iframe.fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
  border: none;
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
