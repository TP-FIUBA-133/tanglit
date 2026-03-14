import * as vscode from "vscode";
import * as tanglit from "./tanglit";

export class SlidesPreviewManager {
  private panel: vscode.WebviewPanel | undefined;
  private currentTheme = "black";
  private currentCodeTheme = "monokai";

  get theme(): string {
    return this.currentTheme;
  }

  get codeTheme(): string {
    return this.currentCodeTheme;
  }

  constructor(private context: vscode.ExtensionContext) {}

  show(document: vscode.TextDocument): void {
    if (this.panel) {
      this.panel.reveal(vscode.ViewColumn.Beside);
    } else {
      this.panel = vscode.window.createWebviewPanel(
        "tanglitSlides",
        "Tanglit: Slides",
        vscode.ViewColumn.Beside,
        { enableScripts: true }
      );
      this.panel.onDidDispose(() => {
        this.panel = undefined;
      });
      this.panel.webview.onDidReceiveMessage((message) => {
        if (message.type === "themeChange") {
          this.currentTheme = message.theme;
          this.currentCodeTheme = message.codeTheme;
          this.update(document);
        }
      });
    }
    this.renderContent(document);
  }

  update(document: vscode.TextDocument): void {
    if (!this.panel) return;
    this.renderContent(document);
  }

  private renderContent(document: vscode.TextDocument): void {
    if (!this.panel) return;
    const text = document.getText();
    try {
      const slidesHtml = tanglit.previewSlides(
        text,
        this.currentTheme,
        this.currentCodeTheme
      );
      this.panel.webview.html = this.wrapWithControls(slidesHtml);
    } catch {
      // Ignore render errors during typing
    }
  }

  private wrapWithControls(slidesHtml: string): string {
    const slideThemes = [
      "black",
      "white",
      "league",
      "beige",
      "sky",
      "night",
      "solarized",
    ];
    const codeThemes = [
      "default",
      "monokai",
      "github",
      "github-dark",
      "agate",
      "ascetic",
    ];

    const slideOptions = slideThemes
      .map(
        (t) =>
          `<option value="${t}" ${t === this.currentTheme ? "selected" : ""}>${t}</option>`
      )
      .join("");
    const codeOptions = codeThemes
      .map(
        (t) =>
          `<option value="${t}" ${t === this.currentCodeTheme ? "selected" : ""}>${t}</option>`
      )
      .join("");

    return `<!DOCTYPE html>
<html>
<head>
  <style>
    body { margin: 0; padding: 0; }
    .controls { padding: 8px; background: #1e1e1e; color: #ccc; font-family: sans-serif; font-size: 13px; display: flex; gap: 12px; align-items: center; }
    .controls select { background: #333; color: #ccc; border: 1px solid #555; padding: 2px 6px; }
    .controls label { margin-right: 4px; }
    iframe { width: 100%; height: calc(100vh - 40px); border: none; }
  </style>
</head>
<body>
  <div class="controls">
    <label>Slide theme:</label>
    <select id="slideTheme">${slideOptions}</select>
    <label>Code theme:</label>
    <select id="codeTheme">${codeOptions}</select>
  </div>
  <iframe id="preview" srcdoc="${this.escapeHtml(slidesHtml)}"></iframe>
  <script>
    const vscode = acquireVsCodeApi();
    document.getElementById('slideTheme').addEventListener('change', (e) => {
      vscode.postMessage({
        type: 'themeChange',
        theme: e.target.value,
        codeTheme: document.getElementById('codeTheme').value
      });
    });
    document.getElementById('codeTheme').addEventListener('change', (e) => {
      vscode.postMessage({
        type: 'themeChange',
        theme: document.getElementById('slideTheme').value,
        codeTheme: e.target.value
      });
    });
  </script>
</body>
</html>`;
  }

  private escapeHtml(html: string): string {
    return html.replace(/&/g, "&amp;").replace(/"/g, "&quot;");
  }
}
