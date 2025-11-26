<script setup lang="ts">
import { ref, watch } from "vue";

const { slides_html } = defineProps<{ slides_html: string }>();

const emit = defineEmits(["change-theme"]);

const availableCodeThemes = [
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

const availableSlideThemes = [
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
  "black-constrast",
  "white-contrast",
];
const selectedSlideTheme = ref("beige"); // Default theme
const selectedCodeTheme = ref("github-dark"); // Default theme

const slide_theme = ref("black");
const code_theme = ref("monokai");

watch(selectedSlideTheme, (new_slide_theme) => {
  slide_theme.value = new_slide_theme;
  emit("change-theme", new_slide_theme, code_theme.value);
});

watch(selectedCodeTheme, (new_code_theme) => {
  code_theme.value = new_code_theme;
  emit("change-theme", slide_theme.value, new_code_theme);
});
</script>

<template>
  <div class="theme-selectors">
    <div class="theme-selector">
      <label for="theme-select">Slide theme: </label>
      <select id="theme-select" v-model="selectedSlideTheme">
        <option v-for="theme in availableSlideThemes" :key="theme" :value="theme">
          {{ theme }}
        </option>
      </select>
    </div>

    <div class="code-theme-selector">
      <label for="code-theme-select">Code theme: </label>
      <select id="code-theme-select" v-model="selectedCodeTheme">
        <option v-for="theme in availableCodeThemes" :key="theme" :value="theme">
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
