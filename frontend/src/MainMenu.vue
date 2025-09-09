<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits(["load_sample_markdown", "file_selected"]);
const fileInput = ref<HTMLInputElement | null>(null); // Template ref for the hidden input

function triggerFileInput() {
  if (!fileInput.value) {
    return;
  }
  fileInput.value.click(); // Programmatically clicks the hidden input
}

function handleFileChange(event: Event) {
  const input_element = event.target as HTMLInputElement;
  const file = input_element?.files?.[0]; // Get the first selected file
  emit("file_selected", file);
  input_element.value = ""; // Reset the input to allow re-selection of the same file
}
</script>

<template>
  <input type="file" ref="fileInput" @change="handleFileChange" style="display: none" accept=".md,.txt" />
  <div>
    <button @click="triggerFileInput" class="custom-file-upload">Open</button>
    <button title="Save">Save</button>
    <button title="Load sample markdown" @click="$emit('load_sample_markdown')">Sample markdown</button>
    <button title="Export slides">Export slides</button>
    <button title="Export to doc">Export doc</button>
    <button title="Tangle code">Tangle code</button>
  </div>
</template>

<style scoped></style>
