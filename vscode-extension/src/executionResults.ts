import * as vscode from "vscode";
import * as tanglit from "./tanglit";

const DECORATION_TYPE = vscode.window.createTextEditorDecorationType({
  after: {
    color: "#888888",
    fontStyle: "italic",
    margin: "0 0 0 1em",
  },
  isWholeLine: true,
});

export class ExecutionResultsManager {
  private outputChannel: vscode.OutputChannel;
  private decorations: Map<
    string,
    { endLine: number; output: tanglit.ExecutionOutput }
  > = new Map();

  constructor() {
    this.outputChannel =
      vscode.window.createOutputChannel("Tanglit");
  }

  showResult(
    editor: vscode.TextEditor,
    blockTag: string,
    blocks: tanglit.CodeBlock[],
    output: tanglit.ExecutionOutput
  ): void {
    const block = blocks.find((b) => b.tag === blockTag);
    if (!block) return;

    this.decorations.set(blockTag, {
      endLine: block.endLine,
      output,
    });

    // Write to output channel
    this.outputChannel.appendLine(`[${blockTag}] stdout: ${output.stdout}`);
    if (output.stderr) {
      this.outputChannel.appendLine(`[${blockTag}] stderr: ${output.stderr}`);
    }
    this.outputChannel.appendLine(
      `[${blockTag}] exit code: ${output.status ?? "unknown"}`
    );
    this.outputChannel.appendLine("---");

    this.updateDecorations(editor);
  }

  updateDecorations(editor: vscode.TextEditor): void {
    const decorations: vscode.DecorationOptions[] = [];

    for (const [, { endLine, output }] of this.decorations) {
      const line = endLine - 1; // VS Code 0-based
      if (line < 0 || line >= editor.document.lineCount) continue;

      const summary =
        output.status === 0
          ? output.stdout.split("\n")[0] || "(no output)"
          : `Error (exit ${output.status}): ${output.stderr.split("\n")[0]}`;

      decorations.push({
        range: new vscode.Range(line, 0, line, 0),
        renderOptions: {
          after: {
            contentText: `  → ${summary}`,
          },
        },
      });
    }

    editor.setDecorations(DECORATION_TYPE, decorations);
  }

  clearForBlock(blockTag: string, editor: vscode.TextEditor): void {
    this.decorations.delete(blockTag);
    this.updateDecorations(editor);
  }

  getResult(blockTag: string): tanglit.ExecutionOutput | undefined {
    return this.decorations.get(blockTag)?.output;
  }
}
