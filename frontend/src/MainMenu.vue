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
  <div>
    <Menu as="div">
      <div>
        <MenuButton> Options </MenuButton>
      </div>
      <MenuItems>
        <MenuItem v-slot="{ active }">
          <a :class="{ 'bg-blue-500': active }" href="/settings"> Account settings </a>
        </MenuItem>
      </MenuItems>
    </Menu>
    <!--    <button @click="$emit('open_file')" class="custom-file-upload">Open</button>-->
    <!--    <button title="Save" @click="$emit('save_file')">Save</button>-->
    <!--    <button title="Load sample markdown" @click="$emit('load_sample_markdown')">Sample markdown</button>-->
    <!--    <button title="Preview slides" @click="$emit('preview_slides')">Preview slides</button>-->
    <!--    <button title="Save slides pdf" @click="$emit('save_slides_pdf')">Save slides pdf</button>-->
    <!--    <button title="Save slides interactive HTML" @click="$emit('save_slides_html')">Save slides html</button>-->
    <!--    <button title="Preview doc" @click="$emit('preview_html')">Preview doc</button>-->
    <!--    <button title="Save the doc as HTML" @click="$emit('save_html')">Save doc as HTML</button>-->
    <!--    <button title="Save the doc as PDF" @click="$emit('save_pdf')">Save doc as PDF</button>-->
    <!--    <button title="Tangle code" @click="$emit('tangle')">Tangle code</button>-->
  </div>
</template>

<style scoped></style>
