<script lang="ts" setup>
import { shallowRef, watch } from "vue";
import VueMonacoEditor from "@guolao/vue-monaco-editor";
import * as monaco from "monaco-editor";

type ICodeEditor = monaco.editor.ICodeEditor;
type IGlyphMarginWidget = monaco.editor.IGlyphMarginWidget;
type IGlyphMarginWidgetPosition = monaco.editor.IGlyphMarginWidgetPosition;

const raw_markdown_mod = defineModel<string>("raw_markdown");
const slide_lines_mod = defineModel<number[]>("slide_lines");
const block_lines_mod = defineModel<number[]>("block_lines");

let margin_glyphs: Record<string, IGlyphMarginWidget> = {};
const emit = defineEmits(["run-block"]);
const MONACO_EDITOR_OPTIONS = {
  automaticLayout: true,
  formatOnType: true,
  formatOnPaste: true,
  glyphMargin: true,
};

const editor = shallowRef<ICodeEditor>();
const handleMount = (editorInstance: ICodeEditor) => (editor.value = editorInstance);

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
    emit("run-block",  line );
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

watch(block_lines_mod, (newValue, oldValue) => {
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
});
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
