<script lang="ts" setup>
import { App, createApp, h, shallowRef, watch } from "vue";
import VueMonacoEditor from "@guolao/vue-monaco-editor";
import * as monaco from "monaco-editor";
import BlockExecutionResult from "./BlockExecutionResult.vue";
import { BlockExecute, Edit } from "./tanglit.ts";

type ICodeEditor = monaco.editor.ICodeEditor;
type IGlyphMarginWidget = monaco.editor.IGlyphMarginWidget;
type IGlyphMarginWidgetPosition = monaco.editor.IGlyphMarginWidgetPosition;

const raw_markdown_mod = defineModel<string>("raw_markdown");
const slide_lines_mod = defineModel<number[]>("slide_lines");
const props = defineProps(["block_lines", "block_execute", "blocks"]);
const zone_ids: Record<number, string> = {};
const zone_apps: Record<number, App> = {};

let margin_glyphs: Record<string, IGlyphMarginWidget> = {};
const emit = defineEmits(["run-block", "add_output_to_markdown"]);
const MONACO_EDITOR_OPTIONS = {
  automaticLayout: true,
  formatOnType: true,
  formatOnPaste: true,
  glyphMargin: true,
};

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

function close_zone(zone_id) {
  if (!editor.value) return;
  editor.value?.changeViewZones((accessor) => {
    accessor.removeZone(zone_id);
    zone_apps[zone_id].unmount();
  });
}

async function makeBlockResult(line_number: number, result: BlockExecute) {
  let viewzone_line = line_number;
  for (let block of props.blocks) {
    // find the end_line of the code block that starts at this line
    // we'll be showing the widget *after* the code block
    if (block.start_line == line_number) {
      let end_line = block.end_line;
      viewzone_line = end_line;
    }
  }
  if (zone_ids[line_number]) {
    // the widget is not dynamic, we need to close the existing one
    // and create another one
    close_zone(zone_ids[line_number]);
  }
  // Create a SINGLE div for everything.
  const domNode = document.createElement("div");
  domNode.className = "block-result-widget";

  // Create and mount a new Vue app onto this div.
  const app = createApp(
    h(BlockExecutionResult, {
      result: result,
      line: line_number,
      onRun_block: () => emit("run-block", line_number),
      onClose: () => close_zone(zone_ids[line_number]),
      onAdd_output_to_markdown: () => emit("add_output_to_markdown", line_number, result.output),
    }),
  );
  app.mount(domNode);

  // Measure the height
  document.body.appendChild(domNode);
  const height = domNode.offsetHeight;
  document.body.removeChild(domNode);
  console.log("Calculated height:", height);

  // Measure the width
  const layoutInfo = editor.value?.getLayoutInfo();
  if (layoutInfo) {
    const availableWidth = layoutInfo.minimap.minimapLeft - layoutInfo.contentLeft;
    domNode.style.width = `${availableWidth}px`;
  }

  // This prevents clicks inside the widget to affect the editor
  domNode.addEventListener("mousedown", (event) => {
    event.stopPropagation();
  });

  // 5. Create the ViewZone using this single div as the domNode.
  const myZoneObject = {
    afterLineNumber: viewzone_line,
    heightInPx: height,
    domNode: domNode, // Use the single div here
    suppressMouseDown: true, // This will now correctly protect the editor
  };

  editor.value?.changeViewZones((accessor) => {
    const zoneId = accessor.addZone(myZoneObject);
    zone_ids[line_number] = zoneId;
    zone_apps[zoneId] = app;
  });
}

function add_output_to_markdown(edit: Edit) {
  editor.value?.executeEdits("embed-result", [
    {
      range: new monaco.Range(edit.start_line, 1, edit.end_line, 1),
      text: edit.content,
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
  padding: 8px;
  z-index: 10000;
  box-sizing: border-box;
  max-width: 800px;
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
