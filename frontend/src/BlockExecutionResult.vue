<script setup lang="ts">
const props = defineProps(["result", "line"]);
console.log("Initial result:", props.result);
defineEmits(["add_output_to_markdown", "run_block", "close"]);
</script>
<template>
  <div class="result">
    <div class="top">
      <span>Block execution</span>
      <button @click="$emit('run_block', line)">Run</button>
      <button class="close" @click="$emit('close')">Close</button>
    </div>
    <div class="error" v-if="props.result.error">{{ result.error }}</div>
    <div class="output-main" v-else-if="result.output">
      <div class="output">
        <div class="output-title">status</div>
        <div :class="['output-content', props.result.output.status ? 'status-error' : 'status-ok']">
          {{ props.result.output.status }}
        </div>
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
.close {
  margin-left: auto;
  background-color: #4a4a4a;
  color: white;
  border: none;
  border-radius: 3pt;
  padding: 2pt 5pt;
  &:hover {
    background-color: #6a6a6a;
  }
  &:active {
    background-color: #2a2a2a;
  }
}

.result {
  display: flex;
  flex-direction: column;
  box-shadow: 0 0 7pt rgba(255, 255, 255, 0.2);
  border: solid 1px #5e5e5e;
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
  padding: 5pt;
}

.output {
  background-color: #333333;
  margin: 2px;
  box-shadow: 0 0 7pt rgba(0, 0, 0, 0.64);
  border: solid 1px #5e5e5e;
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

.output-content.status-ok {
  color: forestgreen;
  border: solid 1px forestgreen;
}

.output-content.status-error {
  color: red;
  border: solid 1px red;
}

.add-to-markdown-btn {
  width: fit-content;
}
</style>
