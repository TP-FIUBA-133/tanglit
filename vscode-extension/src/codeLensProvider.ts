import * as vscode from "vscode";
import * as tanglit from "./tanglit";

export class TanglitCodeLensProvider implements vscode.CodeLensProvider {
  private _onDidChangeCodeLenses: vscode.EventEmitter<void> =
    new vscode.EventEmitter<void>();
  public readonly onDidChangeCodeLenses: vscode.Event<void> =
    this._onDidChangeCodeLenses.event;

  // Track execution results for "Add to Markdown" / "Re-run" CodeLens
  private executionResults: Map<
    string,
    { blockTag: string; output: tanglit.ExecutionOutput; endLine: number }
  > = new Map();

  refresh(): void {
    this._onDidChangeCodeLenses.fire();
  }

  setExecutionResult(
    blockTag: string,
    output: tanglit.ExecutionOutput,
    endLine: number
  ): void {
    this.executionResults.set(blockTag, { blockTag, output, endLine });
    this.refresh();
  }

  clearExecutionResult(blockTag: string): void {
    this.executionResults.delete(blockTag);
    this.refresh();
  }

  provideCodeLenses(document: vscode.TextDocument): vscode.CodeLens[] {
    if (document.languageId !== "markdown") {
      return [];
    }

    const text = document.getText();
    const lenses: vscode.CodeLens[] = [];

    // Code block run buttons
    const blocks = tanglit.parseBlocks(text);
    for (const block of blocks) {
      const line = block.startLine - 1; // VS Code uses 0-based lines
      if (line < 0) continue;
      const range = new vscode.Range(line, 0, line, 0);
      lenses.push(
        new vscode.CodeLens(range, {
          title: "▶ Run Block",
          command: "tanglit.runBlock",
          arguments: [block.tag],
        })
      );

      // If this block has execution results, add "Add to Markdown" and "Re-run" lenses
      const result = this.executionResults.get(block.tag);
      if (result) {
        const endLine = block.endLine - 1;
        const endRange = new vscode.Range(endLine, 0, endLine, 0);
        lenses.push(
          new vscode.CodeLens(endRange, {
            title: "Add to Markdown",
            command: "tanglit.addOutputToMarkdown",
            arguments: [block.tag],
          })
        );
        lenses.push(
          new vscode.CodeLens(endRange, {
            title: "Re-run",
            command: "tanglit.rerunBlock",
            arguments: [block.tag],
          })
        );
      }
    }

    // Slide number indicators
    const slides = tanglit.parseSlides(text);
    for (let i = 0; i < slides.length; i++) {
      const line = slides[i].startLine - 1;
      if (line < 0) continue;
      const range = new vscode.Range(line, 0, line, 0);
      lenses.push(
        new vscode.CodeLens(range, {
          title: `Slide ${i + 1}`,
          command: "",
          arguments: [],
        })
      );
    }

    return lenses;
  }
}
