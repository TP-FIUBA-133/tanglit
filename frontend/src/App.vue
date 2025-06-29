<script setup lang="ts">
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import MarkdownEditor from "./MarkdownEditor.vue";

const exclusion_output = ref("");
const raw_markdown = ref("");
const slides = ref<number[]>([]);
const blocks = ref<number[]>([]);

enum TANGLIT_COMMANDS {
  exclude = "tanglit_exclude",
  parse_slides = "tanglit_parse_slides",
  parse_blocks = "tanglit_parse_blocks",
}

async function exclude(raw_markdown: string): Promise<string> {
  return await invoke(TANGLIT_COMMANDS.exclude, { raw_markdown });
}

async function parse_slides(raw_markdown: string): Promise<number[]> {
  let rv = (await invoke(TANGLIT_COMMANDS.parse_slides, { raw_markdown })) as Array<{ start_line: number }>;
  return rv.map((item) => item.start_line);
}

async function parse_blocks(raw_markdown: string): Promise<number[]> {
  let rv = (await invoke(TANGLIT_COMMANDS.parse_blocks, { raw_markdown })) as Array<{ start_line: number }>;
  return rv.map((item) => item.start_line);
}

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
    slides.value = await parse_slides(newValue);
    exclusion_output.value = await exclude(newValue);
    blocks.value = await parse_blocks(newValue);
  } catch (e) {
    alert("Error: " + e);
  }
  let end_time = performance.now();
  time_to_process.value = Math.floor(end_time - start_time);
});

const fileInput = ref<HTMLInputElement | null>(null); // Template ref for the hidden input
const selectedFileName = ref("No file chosen.");

// This function is called when the custom button is clicked
function triggerFileInput() {
  if (!fileInput.value) {
    return;
  }
  fileInput.value.click(); // Programmatically clicks the hidden input
}

// This function is called when a file is selected in the dialog
function handleFileChange(event: Event) {
  const input_element = event.target as HTMLInputElement;
  const file = input_element?.files?.[0]; // Get the first selected file
  if (file) {
    selectedFileName.value = file.name;
    console.log("Selected file:", file);
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
    input_element.value = ""; // Reset the input to allow re-selection of the same file
  } else {
    selectedFileName.value = "No file chosen.";
  }
}
</script>

<template>
  <main class="container">
    <div class="main-container">
      <div class="editor-wrapper">
        <MarkdownEditor
          v-model:raw_markdown="raw_markdown"
          v-model:slide_lines="slides"
          v-model:block_lines="blocks"
          class="editor"
        />
      </div>
      <div class="exclusion_output">{{ exclusion_output }}</div>
    </div>
    <div class="status-bar">
      <div class="buttons">
        <div>
          <button @click="triggerFileInput" class="custom-file-upload">Open</button>
          <input type="file" ref="fileInput" @change="handleFileChange" style="display: none" accept=".md,.txt" />
        </div>
        <button title="Save">Save</button>
        <button title="Load sample markdown" @click="load_sample_markdown">Sample markdown</button>
        <button title="Export slides">Export slides</button>
        <button title="Export to doc">Export doc</button>
        <button title="Tangle code">Tangle code</button>
      </div>
      <div>Time to process: {{ time_to_process }} ms</div>
    </div>
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

html,
body {
  margin: 0;
  padding: 0;
  height: 100%;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  background-color: #00931f;
  height: 100vh;
}

.main-container {
  display: flex;
  flex-direction: row;
  flex-grow: 1;
  overflow: hidden;
  background-color: #ffffff;
}

.editor-wrapper {
  flex-grow: 1;
  overflow: hidden;
  margin: 0;
}

.exclusion_output {
  width: 50%;
  color: #5d8cec;
  background-color: #222;
  white-space: pre-wrap;
  text-align: left;
  font-family: monospace;
}

.status-bar {
  flex-shrink: 0; /* Prevents the status bar from shrinking */
  padding: 4px 10px;
  border: none;
  margin: 0;
  background-color: #29587e;
  color: white;
  font-family: sans-serif;
  font-size: 12px;
  display: flex;
  flex-direction: row;
  gap: 5px;
  justify-content: center;
  align-items: center;
}

.buttons {
  display: flex;
  flex-direction: row;
  gap: 5px;
}

.buttons button {
  background-color: #003974;
  border-radius: 0;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
