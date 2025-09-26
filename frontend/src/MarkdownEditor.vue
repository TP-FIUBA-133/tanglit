<script lang="ts" setup>
import { createApp, h, ref, shallowRef, watch } from "vue";
import VueMonacoEditor from "@guolao/vue-monaco-editor";
import * as monaco from "monaco-editor";
import BlockExecutionResult from "./BlockExecutionResult.vue";
import { BlockExecute } from "./tanglit.ts";

type ICodeEditor = monaco.editor.ICodeEditor;
type IGlyphMarginWidget = monaco.editor.IGlyphMarginWidget;
type IGlyphMarginWidgetPosition = monaco.editor.IGlyphMarginWidgetPosition;
type IContentWidgetPosition = monaco.editor.IContentWidgetPosition;

const raw_markdown_mod = defineModel<string>("raw_markdown");
const slide_lines_mod = defineModel<number[]>("slide_lines");
const props = defineProps(["block_lines", "block_execute"]);

let margin_glyphs: Record<string, IGlyphMarginWidget> = {};
const emit = defineEmits(["run-block", "add_output_to_markdown"]);
const MONACO_EDITOR_OPTIONS = {
  automaticLayout: true,
  formatOnType: true,
  formatOnPaste: true,
  glyphMargin: true,
};

const my_widget = ref();

const editor = shallowRef<ICodeEditor>();
const handleMount = (editorInstance: ICodeEditor) => (editor.value = editorInstance);
watch(
  () => props.block_execute,
  (new_value, old_value) => {
    console.log("new block_execute: ", new_value);
    console.log("old block_execute: ", old_value);
    makeBlockResult(new_value.line, new_value);
  },
);

function close_widget() {
  if (!editor.value) return;
  console.log("close_widget: ", my_widget.value);
  editor.value.removeContentWidget(my_widget.value.widget);
  my_widget.value.unmount();
  my_widget.value = "";
}

function makeBlockResult(line_number: number, result: BlockExecute) {
  if (my_widget.value) {
    close_widget();
  }
  let widget_dom = document.createElement("div");
  const app = createApp(
    h(BlockExecutionResult, {
      result: result,
      line: line_number,
      onRun_block: () => emit("run-block", line_number),
      onClose: close_widget,
      onAdd_output_to_markdown: () => emit("add_output_to_markdown", line_number, result.output),
    }),
  );

  app.mount(widget_dom);
  widget_dom.className = "block-result-widget";
  let random_suffix = Math.floor(Math.random() * 1000000);
  let widget_id = "block_result_widget" + random_suffix;
  let w = {
    getId: function () {
      return widget_id;
    },
    getDomNode: function () {
      return widget_dom;
    },
    getPosition: function (): IContentWidgetPosition {
      return {
        position: { lineNumber: line_number, column: 1 },
        // Place it below the current line
        preference: [monaco.editor.ContentWidgetPositionPreference.BELOW],
      };
    },
  };
  my_widget.value = { widget: w, unmount: app.unmount, line: line_number };
  editor.value?.addContentWidget(w);
}

function add_output_to_markdown(text: string, line_number: number) {
  console.log("text: ", text);
  console.log("line_number: ", line_number);
  editor.value?.executeEdits("embed-result", [
    {
      range: new monaco.Range(line_number, 1, line_number, 1),
      text: text + "\n", // Add the text on the next line
      forceMoveMarkers: true,
    },
  ]);
}

defineExpose({ makeBlockResult: makeBlockResult, add_output_to_markdown: add_output_to_markdown });

function makeGlyphWidget(line: number, widget_id: string, widget_dom: HTMLElement): IGlyphMarginWidget {
  return {
    getId: function () {
      return widget_id;
    },
    getDomNode: function () {
      return widget_dom;
    },
    getPosition: function (): IGlyphMarginWidgetPosition {
      return {
        range: new monaco.Range(line, 1, line, 1), // Use 'range' instead of 'lineNumber'
        lane: monaco.editor.GlyphMarginLane.Center,
        zIndex: 1000, // Ensure the widget appears above other content
      };
    },
  };
}

function SlideWidget(line: number, slide_idx: number): IGlyphMarginWidget {
  const widgetNode = document.createElement("div");
  widgetNode.innerHTML = slide_idx.toString();
  widgetNode.className = "slide-widget";
  return makeGlyphWidget(line, get_margin_glyph_id(line, "slide"), widgetNode);
}

function RunBlockWidget(line: number): IGlyphMarginWidget {
  const widgetNode = document.createElement("div");
  widgetNode.innerHTML = "<button>▶</button>"; // Use different glyphs for slides and code
  widgetNode.className = "run-block-widget";
  widgetNode.onclick = () => {
    // emit an event to run the block
    // emit("run-block", line);
    makeBlockResult(line, { line: line, output: null, error: null });
  };
  return makeGlyphWidget(line, get_margin_glyph_id(line, "code"), widgetNode);
}

function get_margin_glyph_id(line: number, _type: "slide" | "code"): string {
  return "my.glyph.margin.widget." + _type + "." + line; // Unique ID for the widget
}

function add_margin_glyph(myGlyphWidget: IGlyphMarginWidget) {
  if (!editor.value) return;
  let editorInstance = editor.value;
  editorInstance.addGlyphMarginWidget(myGlyphWidget);
  margin_glyphs[myGlyphWidget.getId()] = myGlyphWidget;
}

watch(slide_lines_mod, (newValue, oldValue) => {
  if (!editor.value) return;
  let editorInstance = editor.value;

  const newLines = new Set(newValue || []);
  const oldLines = new Set(oldValue || []);
  oldValue?.forEach((line) => {
    if (line < 1) return; // Ensure line numbers are valid
    if (!newLines.has(line)) {
      const marginGlyph = margin_glyphs[get_margin_glyph_id(line, "slide")];
      editorInstance.removeGlyphMarginWidget(marginGlyph);
    }
  });
  newValue?.forEach((line, idx) => {
    if (line < 1) return; // Ensure line numbers are valid
    if (!oldLines.has(line)) {
      // Add the margin glyph only if it doesn't already exist
      add_margin_glyph(SlideWidget(line, idx + 1)); // Pass the slide index as extra data
    }
  });
});

watch(
  () => props.block_lines,
  (newValue, oldValue) => {
    if (!editor.value) return;
    let editorInstance = editor.value;

    const newLines = new Set(newValue || []);
    const oldLines = new Set(oldValue || []);

    oldValue?.forEach((line) => {
      if (line < 1) return; // Ensure line numbers are valid
      if (!newLines.has(line)) {
        const marginGlyph = margin_glyphs[get_margin_glyph_id(line, "code")];
        editorInstance.removeGlyphMarginWidget(marginGlyph);
      }
    });
    newValue?.forEach((line) => {
      if (line < 1) return; // Ensure line numbers are valid
      if (!oldLines.has(line)) {
        // Add the margin glyph only if it doesn't already exist
        add_margin_glyph(RunBlockWidget(line)); // Pass the slide index as extra data
      }
    });
  },
);
</script>

<template>
  <VueMonacoEditor
    class="vue-monaco-editor"
    v-model:value="raw_markdown_mod"
    theme="vs-dark"
    :options="MONACO_EDITOR_OPTIONS"
    @mount="handleMount"
    language="markdown"
  />
</template>

<style>
.slide-widget {
  color: orange;
  font-size: 0.875rem;
  text-align: center;
  line-height: 19px;
  align-content: center;
}

.run-block-widget {
  padding: 0;
}

.block-result-widget {
  background-color: #272727;
  border: 1px solid #5e5e5e;
  padding: 8px;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  min-width: 20em;
  z-index: 10000;
}

.run-block-widget button {
  color: green;
  padding: 0;
  margin: 0;
  box-shadow: none;
  border: none;
  width: 100%;
  height: 100%;
  border-radius: 0;
  display: block;

  font-size: 0.875rem;
  background-color: transparent;
  text-align: center;
  line-height: 19px;
  align-content: center;
}

.vue-monaco-editor {
  width: 100%;
  height: 100%;
}
</style>
