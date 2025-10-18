<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import Reveal from "reveal.js";
import "reveal.js/dist/reveal.css";
import Markdown from "reveal.js/plugin/markdown/markdown.esm.js";
import Highlight from "reveal.js/plugin/highlight/highlight.esm.js";

const { slides_markdown } = defineProps<{ slides_markdown: string[] }>();

let deck: Reveal.Api | null = null;

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

const slideThemeLinkId = "slide-theme-link";
const codeThemeLinkId = "code-theme-link";

function initReveal() {
  deck = new Reveal({
    plugins: [Markdown, Highlight],
    embedded: true,
    hash: true,
    // -- Adjust these values --
    width: 1920,
    height: 1080,
    margin: 0.04, // A bit of breathing room

    // Optional: Set scaling limits
    minScale: 0.2,
    maxScale: 2.0,
    center: false, // Disables vertical centering
  });

  deck.on("ready", () => {
    document.querySelectorAll(".reveal section h1, .reveal section h2").forEach((el) => el.classList.add("r-fit-text"));
  });

  // Also maybe on slidechanged event:
  deck.on("slidechanged", (ev: Event) => {
    const { currentSlide } = ev as unknown as { currentSlide?: HTMLElement };
    currentSlide?.querySelectorAll<HTMLElement>("h1, h2").forEach((el) => el.classList.add("r-fit-text"));
  });

  deck.initialize();
}

onMounted(() => {
  if (!document.getElementById(slideThemeLinkId)) {
    const link = document.createElement("link");
    link.id = slideThemeLinkId;
    link.rel = "stylesheet";
    link.href = `/node_modules/reveal.js/dist/theme/${selectedSlideTheme.value}.css`;
    document.head.appendChild(link);
  }

  if (!document.getElementById(codeThemeLinkId)) {
    const link = document.createElement("link");
    link.id = codeThemeLinkId;
    link.rel = "stylesheet";
    link.href = `/node_modules/highlight.js/styles/${selectedCodeTheme.value}.css`;
    document.head.appendChild(link);
  }

  initReveal();
});

watch(selectedSlideTheme, (newTheme) => {
  const themeLink = document.getElementById(slideThemeLinkId) as HTMLLinkElement;
  if (themeLink) {
    themeLink.href = `/node_modules/reveal.js/dist/theme/${newTheme}.css`;
  }
});

watch(selectedCodeTheme, (newTheme) => {
  const themeLink = document.getElementById(codeThemeLinkId) as HTMLLinkElement;
  if (themeLink) {
    themeLink.href = `/node_modules/highlight.js/styles/${newTheme}.css`;
  }
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
  <div class="reveal">
    <div class="slides">
      <section v-for="(md, index) in slides_markdown" :key="index" data-markdown>
        <div data-template v-html="md"></div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.reveal {
  font-family: sans-serif !important;
  height: 100%;
  width: 100%;
}

.reveal .slides {
  /* This overrides the theme's default text-align: center */
  text-align: left;
}

.theme-selectors {
  display: flex;
  flex-direction: row;
}
</style>
