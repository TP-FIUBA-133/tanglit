import * as vscode from "vscode";
import * as tanglit from "./tanglit";

export class HtmlPreviewManager {
  private panel: vscode.WebviewPanel | undefined;
  private currentTheme = "pico";

  get theme(): string {
    return this.currentTheme;
  }

  constructor(private context: vscode.ExtensionContext) {}

  show(document: vscode.TextDocument): void {
    if (this.panel) {
      this.panel.reveal(vscode.ViewColumn.Beside);
    } else {
      this.panel = vscode.window.createWebviewPanel(
        "tanglitHtml",
        "Tanglit: HTML",
        vscode.ViewColumn.Beside,
        { enableScripts: true }
      );
      this.panel.onDidDispose(() => {
        this.panel = undefined;
      });
      this.panel.webview.onDidReceiveMessage((message) => {
        if (message.type === "themeChange") {
          this.currentTheme = message.theme;
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
      const html = tanglit.previewHtml(text, this.currentTheme);
      this.panel.webview.html = this.wrapWithControls(html);
    } catch {
      // Ignore render errors during typing
    }
  }

  private wrapWithControls(contentHtml: string): string {
    const themes = ["pico", "water", "sakura", "latex"];
    const options = themes
      .map(
        (t) =>
          `<option value="${t}" ${t === this.currentTheme ? "selected" : ""}>${t}</option>`
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
    <label>Theme:</label>
    <select id="htmlTheme">${options}</select>
  </div>
  <iframe id="preview" srcdoc="${this.escapeHtml(contentHtml)}"></iframe>
  <script>
    const vscode = acquireVsCodeApi();
    document.getElementById('htmlTheme').addEventListener('change', (e) => {
      vscode.postMessage({
        type: 'themeChange',
        theme: e.target.value
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
