<script setup lang="ts">
const props = defineProps(["result", "line"]);
console.log("Initial result:", props.result);
defineEmits(["add_output_to_markdown", "run_block", "close"]);
</script>
<template>
  <div class="result">
    <button class="close" @click="$emit('close')">Close</button>
    <div class="top">
      <span>Block execution</span>
      <button @click="$emit('run_block', line)">Run</button>
    </div>
    <div class="error" v-if="props.result.error"><span>Error</span>{{ result.error }}</div>
    <div class="output-main" v-else-if="result.output">
      <div class="output">
        <div class="output-title">status</div>
        <div class="output-content">{{ props.result.output.status }}</div>
      </div>
      <div class="output">
        <div class="output-title">stdout</div>
        <div class="output-content">{{ props.result.output.stdout }}</div>
      </div>
      <div class="output" v-if="props.result.output.stderr">
        <div class="output-title">stderr</div>
        <div class="output-content">{{ props.result.output.stderr }}</div>
      </div>
      <button class="add-to-markdown-btn" @click="$emit('add_output_to_markdown', props.result.output)">
        Add to markdown
      </button>
    </div>
  </div>
</template>

<style scoped>
.top {
  display: flex;
  align-items: center;
  margin-bottom: 5pt;
  gap: 10pt;
}
.close {
  position: absolute;
  top: 5pt;
  right: 20pt;
  background-color: #4a4a4a;
  color: white;
  border: none;
  border-radius: 3pt;
  padding: 2pt 5pt;
}

.result {
  display: flex;
  flex-direction: column;
  background-color: #2b2b2b;
  color: #d3d3d3;
  width: calc(100% - 20pt);
  padding: 5pt;
}

.output-main {
  gap: 5pt;
  display: flex;
  flex-direction: column;
}

.error {
  font-family: monospace;
  background-color: darkred;
  color: white;
  white-space: pre-wrap;
  justify-content: left;
  text-align: left;
}

.output {
  background-color: #333333;
  margin: 2px;
}

.output-title {
  text-align: left;
  font-family: sans-serif;
  padding: 5px;
}

.output-content {
  font-family: monospace;
  background-color: #2b2727;
  color: white;
  white-space: pre-wrap;
  justify-content: left;
  text-align: left;
  padding: 5px;
}

.add-to-markdown-btn {
  width: fit-content;
}
</style>
