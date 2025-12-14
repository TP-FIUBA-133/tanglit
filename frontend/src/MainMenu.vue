<script setup lang="ts">
defineEmits([
  "load_sample_markdown",
  "file_selected",
  "preview_slides",
  "preview_html",
  "save_file",
  "open_file",
  "save_html",
  "save_slides_pdf",
  "save_slides_html",
  "save_pdf",
  "tangle",
]);
</script>

<template>
  <div class="menu">
    <button @click="$emit('open_file')" class="custom-file-upload">Open</button>
    <button title="Save" @click="$emit('save_file')">Save</button>
    <button title="Load sample markdown" @click="$emit('load_sample_markdown')">Sample markdown</button>
    <button title="Preview slides" @click="$emit('preview_slides')">Preview slides</button>
    <button title="Save slides pdf" @click="$emit('save_slides_pdf')">Save slides pdf</button>
    <button title="Save slides interactive HTML" @click="$emit('save_slides_html')">Save slides html</button>
    <button title="Preview doc" @click="$emit('preview_html')">Preview doc</button>
    <button title="Save the doc as HTML" @click="$emit('save_html')">Save doc as HTML</button>
    <button title="Save the doc as PDF" @click="$emit('save_pdf')">Save doc as PDF</button>
    <button title="Tangle code" @click="$emit('tangle')">Tangle code</button>
  </div>
</template>

<style scoped>
.menu {
  display: flex;
  gap: 5pt;
  padding: 3pt;
  background-color: rgb(255, 255, 255);
  box-shadow: rgb(0, 0, 0) 0px -1px 17px;
  z-index: 100;
}

button {
  border: none;
  background-color: #5d8cec;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.75);
  color: #ffffff;
  padding: 8px 12px;
  border-radius: 4px;
  font-size: 14px;
  &:hover {
    background-color: #295ccc;
  }
  &:active {
    translate: 2px 2px;
    background-color: #1a3a66;
    box-shadow: none;
  }
}
</style>
