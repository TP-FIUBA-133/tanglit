<script setup lang="ts">
import { BlockExecute } from "./tanglit.ts";

const { result } = defineProps<{ result: BlockExecute }>();
</script>
<template>
  <div class="result">
    Block execution
    <div class="error" v-if="result.error"><span>Error</span>{{ result.error }}</div>
    <div v-else-if="result.output">
      <div class="output">
        <div class="output-title">status</div>
        <div class="output-content">{{ result.output.status }}</div>
      </div>
      <div class="output">
        <div class="output-title">stdout</div>
        <div class="output-content">{{ result.output.stdout }}</div>
      </div>
      <div class="output">
        <div class="output-title">stderr</div>
        <div class="output-content">{{ result.output.stderr }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.result {
  display: flex;
  flex-direction: column;
  background-color: #006eb3;
  color: white;
  gap: 5px;
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
  background-color: #00304e;
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
</style>
