<script setup lang="ts">
import SlideView from "./SlideView.vue";
import sum from "hash-sum";

const { slides_markdown } = defineProps<{ slides_markdown: string[] }>();

function make_key(slides_markdown: string[]): string {
  let start = performance.now();
  let y = sum(slides_markdown);
  let end = performance.now();
  console.log("hash took: ", end - start);
  return y;
}
</script>

<!--We use this wrapper over SlideView to hide the :key part which is not very intuitive, -->
<!--but it's the way to force a re-render of SlideView when slides_markdown changes.-->

<template>
  <SlideView :slides_markdown="slides_markdown" :key="make_key(slides_markdown)" />
</template>

<style scoped></style>
