<script setup lang="ts">
import { computed, Ref, ref, watch } from "vue";
import MarkdownEditor from "./MarkdownEditor.vue";
import * as tanglit from "./tanglit.ts";
import { BlockExecute, Edit } from "./tanglit.ts";
import MainMenu from "./MainMenu.vue";
import "splitpanes/dist/splitpanes.css";
// @ts-expect-error missing types
import { Pane, Splitpanes } from "splitpanes";
import SlideViewMain from "./SlideViewMain.vue";
import HtmlPreview from "./HtmlPreview.vue";
import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import { open, save } from "@tauri-apps/plugin-dialog";
import { useToast } from "vue-toastification";

const toast = useToast();

const exclusion_output = ref("");
const raw_markdown = ref("");
const slides = ref<number[]>([]);
const slides_markdown = ref<string[]>([]);
const all_blocks = ref<{ start_line: number; tag: string }[]>([]);
const block_execute = ref<BlockExecute>({ error: undefined, output: undefined });
const html_preview = ref("");
const currentFilePath: Ref<string | null> = ref(null);

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

async function openFile() {
  try {
    const selectedPath = await open({
      multiple: false, // Only allow selection of a single file
      title: "Open your Tanglit document",
      filters: [
        {
          name: "Markdown/Text",
          extensions: ["md"], // Customize as needed
        },
      ],
    });

    if (typeof selectedPath !== "string") {
      // User cancelled, or something unexpected happened (shouldn't be array with multiple: false)
      currentFilePath.value = null;
      return;
    }

    currentFilePath.value = selectedPath;

    const content = await readTextFile(selectedPath);
    // Update the editor content
    raw_markdown.value = content;
    toast.success(`Successfully opened and read file: ${currentFilePath.value}`);
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);
    toast.error(`Error opening file: ${message}`);
    currentFilePath.value = null;
  }
}

async function save_file() {
  if (!currentFilePath.value) {
    currentFilePath.value = await save();
  }
  if (!currentFilePath.value) return;

  writeTextFile(currentFilePath.value, raw_markdown.value)
    .then(() => {
      toast.success(`Saved file ${currentFilePath.value}`);
    })
    .catch((error: string) => {
      toast.error(`Error saving file: ${error}`);
    });
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

async function preview_html() {
  await tanglit.preview_html(raw_markdown.value).then((html: string) => {
    html_preview.value = html;
  });
}

async function save_html() {
  let html_save_path: string | null = await save();
  if (!html_save_path) return;
  writeTextFile(html_save_path, html_preview.value)
    .then(() => {
      toast.success(`Saved file ${html_save_path}`);
    })
    .catch((error: string) => {
      toast.error(`Error saving file: ${error}`);
    });
}

async function save_pdf() {
  let pdf_save_path: string | null = await save();
  if (!pdf_save_path) return;
  await tanglit.save_pdf(raw_markdown.value, pdf_save_path);
}

const markdown_editor = ref<InstanceType<typeof MarkdownEditor> | null>(null);

async function add_output_to_markdown(block_line: number, output: string) {
  const editor = markdown_editor.value;
  if (!editor) return;
  console.log("Adding output to markdown:", output);
  let block_name = "";
  console.log("Run block at line:", block_line);
  // find the corresponding block name
  for (let i = 0; i < all_blocks.value.length; i++) {
    const block = all_blocks.value[i];
    if (block.start_line == block_line) {
      // Here you can execute the block or do whatever you need with it
      block_name = block.tag;
      break;
    }
  }

  let edit: Edit = await tanglit.format_output(raw_markdown.value, block_name, output);
  editor.add_output_to_markdown(edit);
}

const block_lines = computed(() => all_blocks.value.map((item) => item.start_line));
</script>

<template>
  <main class="container">
    <div class="main-container">
      <splitpanes vertical class="default-theme">
        <pane min-size="50" class="editor-wrapper">
          <MarkdownEditor
            ref="markdown_editor"
            @run-block="run_block"
            v-model:raw_markdown="raw_markdown"
            v-model:slide_lines="slides"
            :block_lines="block_lines"
            :blocks="all_blocks"
            :block_execute="block_execute"
            v-on:add_output_to_markdown="add_output_to_markdown"
            class="editor"
          />
        </pane>
        <pane min-size="30">
          <splitpanes horizontal class="default-theme">
            <pane min-size="30">
              <SlideViewMain class="slide-view" :slides_markdown="slides_markdown" />
            </pane>
            <pane min-size="30" v-if="html_preview">
              <HtmlPreview :html="html_preview" />
            </pane>
          </splitpanes>
        </pane>
      </splitpanes>
    </div>
    <MainMenu
      v-on:load_sample_markdown="load_sample_markdown"
      v-on:preview_slides="preview_slides"
      v-on:preview_html="preview_html"
      v-on:open_file="openFile"
      v-on:save_file="save_file"
      v-on:save_html="save_html"
      v-on:save_pdf="save_pdf"
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
