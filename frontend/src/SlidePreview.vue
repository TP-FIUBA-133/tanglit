<script setup lang="ts">
import { onMounted, ref, watch } from "vue";

const { slides_html } = defineProps<{ slides_html: string }>();

const emit = defineEmits(["change-theme"]);

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

const main_theme = ref("black"); // Default theme
const code_theme = ref("monokai"); // Default theme

function refresh() {
  emit("change-theme", main_theme.value, code_theme.value);
}

watch(main_theme, refresh);

watch(code_theme, refresh);

onMounted(refresh);
</script>

<template>
  <div class="theme-selectors">
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
  <iframe :srcdoc="slides_html"></iframe>
</template>

<style scoped>
iframe {
  height: 100%;
  width: 100%;
}
</style>
