<script setup lang="ts">
defineEmits(["load_sample_markdown", "file_selected", "preview_slides", "preview_html", "save_file", "open_file"]);
</script>

<template>
  <div>
    <button @click="$emit('open_file')" class="custom-file-upload">Open</button>
    <button title="Save" @click="$emit('save_file')">Save</button>
    <button title="Load sample markdown" @click="$emit('load_sample_markdown')">Sample markdown</button>
    <button title="Preview slides" @click="$emit('preview_slides')">Preview slides</button>
    <button title="Export to doc" @click="$emit('preview_html')">Export doc</button>
    <button title="Tangle code">Tangle code</button>
  </div>
</template>

<style scoped></style>
