<script setup lang="ts">
import { computed, ref, watch } from "vue";
import MarkdownEditor from "./MarkdownEditor.vue";
import { BlockExecute } from "./tanglit.ts";
import * as tanglit from "./tanglit.ts";
import BlockExecutionResult from "./BlockExecutionResult.vue";
import MainMenu from "./MainMenu.vue";
import { Splitpanes, Pane } from "splitpanes";
import "splitpanes/dist/splitpanes.css";
import SlideViewMain from "./SlideViewMain.vue";

const exclusion_output = ref("");
const raw_markdown = ref("");
const slides = ref<number[]>([]);
const slides_markdown = ref<string[]>([]);
const all_blocks = ref<{ start_line: number; tag: string }[]>([]);
const block_execute = ref<BlockExecute>({ error: undefined, result: undefined });

function load_sample_markdown() {
  fetch("/src/assets/example.md")
    .then((response) => response.text())
    .then((text) => {
      raw_markdown.value = text;
      console.log("Sample markdown loaded.");
    })
    .catch((error) => {
      console.error("Error loading sample markdown:", error);
    });
}

const time_to_process = ref(0);

watch(raw_markdown, async (newValue) => {
  let start_time = performance.now();
  try {
    slides.value = await tanglit.parse_slides(newValue);
    exclusion_output.value = await tanglit.exclude(newValue);
    all_blocks.value = await tanglit.parse_blocks(newValue);
  } catch (e) {
    alert("Error: " + e);
  }
  let end_time = performance.now();
  time_to_process.value = Math.floor(end_time - start_time);
});

const selectedFileName = ref("No file chosen.");

// This function is called when a file is selected in the dialog
function file_selected(file) {
  if (file) {
    selectedFileName.value = file.name;
    // You can now read the file or do whatever you need with it
    const reader = new FileReader();
    reader.onload = (e) => {
      const result = e.target?.result;
      if (typeof result === "string") {
        raw_markdown.value = result; // Set the content of the editor only if defined
      }
    };
    reader.onerror = (e) => {
      console.error("Error reading file:", e);
      alert("Error reading file: " + e);
    };
    reader.readAsText(file); // Read the file as text
  } else {
    selectedFileName.value = "No file chosen.";
  }
}

async function run_block(line: number) {
  console.log("Run block at line:", line);
  // find the corresponding block name
  for (let i = 0; i < all_blocks.value.length; i++) {
    const block = all_blocks.value[i];
    if (block.start_line == line) {
      // Here you can execute the block or do whatever you need with it
      block_execute.value = await tanglit.execute_block(raw_markdown.value, block.tag);
      break;
    }
  }
}

async function preview_slides() {
  slides_markdown.value = await tanglit.gen_slides(raw_markdown.value);
  console.log("Slides generated:", slides_markdown.value);
}

const block_lines = computed(() => all_blocks.value.map((item) => item.start_line));
</script>

<template>
  <main class="container">
    <div class="main-container">
      <splitpanes vertical class="default-theme">
        <pane min-size="50" class="editor-wrapper">
          <MarkdownEditor
            @run-block="run_block"
            v-model:raw_markdown="raw_markdown"
            v-model:slide_lines="slides"
            :block_lines="block_lines"
            class="editor"
          />
        </pane>
        <pane min-size="30">
          <splitpanes horizontal class="default-theme">
            <pane min-size="30">
              <SlideViewMain class="slide-view" :slides_markdown="slides_markdown" />
            </pane>
            <pane min-size="30">
              <BlockExecutionResult :result="block_execute" />
            </pane>
          </splitpanes>
        </pane>
      </splitpanes>
    </div>
    <MainMenu
      v-on:load_sample_markdown="load_sample_markdown"
      v-on:file_selected="file_selected"
      v-on:preview_slides="preview_slides"
    />
  </main>
</template>

<style lang="scss">
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #0f0f0f;
  background-color: #f6f6f6;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.slide-view {
  height: 100%;
  width: 600px;
}

html,
body {
  margin: 0;
  padding: 0;
  height: 100%;
  overflow: hidden; /* Prevent body scrollbars */
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  height: 100vh; /* Fill the entire viewport height */
}

.main-container {
  display: flex;
  flex-direction: row;
  flex-grow: 1;
  overflow: hidden;
  background-color: #ffffff;
}

.exclusion_output {
  width: 100%;
  color: #5d8cec;
  background-color: #222;
  white-space: pre-wrap;
  text-align: left;
  font-family: monospace;
}
</style>
