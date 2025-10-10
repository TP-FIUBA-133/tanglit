<script setup lang="ts">
import { onMounted } from "vue";
import Reveal from "reveal.js";
import "reveal.js/dist/reveal.css";
import "reveal.js/dist/theme/black.css"; // you can pick another theme
import Markdown from "reveal.js/plugin/markdown/markdown.esm.js";
import Highlight from "reveal.js/plugin/highlight/highlight.esm.js";
import "reveal.js/plugin/highlight/monokai.css";

const { slides_markdown } = defineProps<{ slides_markdown: string[] }>();

let deck: Reveal.Api | null = null;

function initReveal() {
  // Reveal.initialize({
  //   plugins: [ RevealMarkdown,  ],
  //
  // });

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
  deck.on("slidechanged", (event: any) => {
    event.currentSlide
  .querySelectorAll("h1, h2")
  .forEach((el: HTMLElement) => el.classList.add("r-fit-text"));

  });

  deck.initialize();
}

onMounted(() => {
  initReveal();
});
</script>

<template>
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
</style>
