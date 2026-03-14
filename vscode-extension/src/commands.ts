import * as vscode from "vscode";
import * as tanglit from "./tanglit";
import { TanglitCodeLensProvider } from "./codeLensProvider";
import { SlidesPreviewManager } from "./slidesPreview";
import { HtmlPreviewManager } from "./htmlPreview";
import { ExecutionResultsManager } from "./executionResults";

export function registerCommands(
  context: vscode.ExtensionContext,
  codeLensProvider: TanglitCodeLensProvider,
  slidesPreview: SlidesPreviewManager,
  htmlPreview: HtmlPreviewManager,
  executionResults: ExecutionResultsManager
): void {
  // Run a code block
  context.subscriptions.push(
    vscode.commands.registerCommand(
      "tanglit.runBlock",
      async (blockTag: string) => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;

        const text = editor.document.getText();
        try {
          const output = tanglit.executeBlock(text, blockTag);
          const blocks = tanglit.parseBlocks(text);
          executionResults.showResult(editor, blockTag, blocks, output);
          codeLensProvider.setExecutionResult(
            blockTag,
            output,
            blocks.find((b) => b.tag === blockTag)?.endLine ?? 0
          );
        } catch (e) {
          vscode.window.showErrorMessage(
            `Tanglit: Error executing block "${blockTag}": ${e}`
          );
        }
      }
    )
  );

  // Re-run a block
  context.subscriptions.push(
    vscode.commands.registerCommand(
      "tanglit.rerunBlock",
      async (blockTag: string) => {
        await vscode.commands.executeCommand("tanglit.runBlock", blockTag);
      }
    )
  );

  // Add output to markdown
  context.subscriptions.push(
    vscode.commands.registerCommand(
      "tanglit.addOutputToMarkdown",
      async (blockTag: string) => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;

        const output = executionResults.getResult(blockTag);
        if (!output) return;

        const text = editor.document.getText();
        try {
          const edit = tanglit.formatOutput(text, blockTag, output);
          const workspaceEdit = new vscode.WorkspaceEdit();
          const startLine = edit.startLine - 1; // VS Code 0-based
          const endLine = edit.endLine - 1;
          const range = new vscode.Range(startLine, 0, endLine, 0);
          workspaceEdit.replace(
            editor.document.uri,
            range,
            edit.content + "\n"
          );
          await vscode.workspace.applyEdit(workspaceEdit);
          executionResults.clearForBlock(blockTag, editor);
          codeLensProvider.clearExecutionResult(blockTag);
        } catch (e) {
          vscode.window.showErrorMessage(
            `Tanglit: Error adding output: ${e}`
          );
        }
      }
    )
  );

  // Preview slides
  context.subscriptions.push(
    vscode.commands.registerCommand("tanglit.previewSlides", () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;
      slidesPreview.show(editor.document);
    })
  );

  // Preview HTML
  context.subscriptions.push(
    vscode.commands.registerCommand("tanglit.previewHtml", () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;
      htmlPreview.show(editor.document);
    })
  );

  // Tangle all
  context.subscriptions.push(
    vscode.commands.registerCommand("tanglit.tangleAll", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;

      const folder = await vscode.window.showOpenDialog({
        canSelectFolders: true,
        canSelectFiles: false,
        canSelectMany: false,
        openLabel: "Select output directory",
      });
      if (!folder || folder.length === 0) return;

      try {
        const count = tanglit.tangle(
          editor.document.getText(),
          folder[0].fsPath
        );
        vscode.window.showInformationMessage(
          `Tanglit: Tangled ${count} file(s) to ${folder[0].fsPath}`
        );
      } catch (e) {
        vscode.window.showErrorMessage(`Tanglit: Error tangling: ${e}`);
      }
    })
  );

  // Export PDF (via Puppeteer)
  context.subscriptions.push(
    vscode.commands.registerCommand("tanglit.exportPdf", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;

      const savePath = await vscode.window.showSaveDialog({
        filters: { PDF: ["pdf"] },
      });
      if (!savePath) return;

      try {
        const html = tanglit.previewHtml(
          editor.document.getText(),
          "pico"
        );
        await htmlToPdf(html, savePath.fsPath, false);
        vscode.window.showInformationMessage(
          `Tanglit: PDF saved to ${savePath.fsPath}`
        );
      } catch (e) {
        vscode.window.showErrorMessage(`Tanglit: Error exporting PDF: ${e}`);
      }
    })
  );

  // Export Slides PDF
  context.subscriptions.push(
    vscode.commands.registerCommand("tanglit.exportSlidesPdf", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;

      const savePath = await vscode.window.showSaveDialog({
        filters: { PDF: ["pdf"] },
      });
      if (!savePath) return;

      try {
        const html = tanglit.previewSlides(
          editor.document.getText(),
          "black",
          "monokai"
        );
        await htmlToPdf(html, savePath.fsPath, true);
        vscode.window.showInformationMessage(
          `Tanglit: Slides PDF saved to ${savePath.fsPath}`
        );
      } catch (e) {
        vscode.window.showErrorMessage(
          `Tanglit: Error exporting slides PDF: ${e}`
        );
      }
    })
  );

  // Export menu (quick-pick)
  context.subscriptions.push(
    vscode.commands.registerCommand("tanglit.exportMenu", async () => {
      const choice = await vscode.window.showQuickPick(
        [
          { label: "HTML Document", command: "tanglit.exportHtml" },
          { label: "PDF Document", command: "tanglit.exportPdf" },
          { label: "Slides (HTML)", command: "tanglit.exportSlidesHtml" },
          { label: "Slides (PDF)", command: "tanglit.exportSlidesPdf" },
          { label: "Tangle All", command: "tanglit.tangleAll" },
        ],
        { placeHolder: "Export as..." }
      );
      if (choice) {
        if (choice.command === "tanglit.exportHtml") {
          await exportHtml();
        } else if (choice.command === "tanglit.exportSlidesHtml") {
          await exportSlidesHtml();
        } else {
          await vscode.commands.executeCommand(choice.command);
        }
      }
    })
  );

  // Context menu: Run This Block
  context.subscriptions.push(
    vscode.commands.registerCommand("tanglit.runThisBlock", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;

      const text = editor.document.getText();
      const blocks = tanglit.parseBlocks(text);
      const cursorLine = editor.selection.active.line + 1; // 1-based

      const block = blocks.find(
        (b) => cursorLine >= b.startLine && cursorLine <= b.endLine
      );
      if (block) {
        await vscode.commands.executeCommand("tanglit.runBlock", block.tag);
      } else {
        vscode.window.showWarningMessage(
          "Tanglit: Cursor is not inside a code block"
        );
      }
    })
  );

  // Context menu: Tangle This Block
  context.subscriptions.push(
    vscode.commands.registerCommand("tanglit.tangleThisBlock", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) return;

      const text = editor.document.getText();
      const blocks = tanglit.parseBlocks(text);
      const cursorLine = editor.selection.active.line + 1;

      const block = blocks.find(
        (b) => cursorLine >= b.startLine && cursorLine <= b.endLine
      );
      if (block && block.export) {
        const folder = await vscode.window.showOpenDialog({
          canSelectFolders: true,
          canSelectFiles: false,
          canSelectMany: false,
          openLabel: "Select output directory",
        });
        if (!folder || folder.length === 0) return;
        const count = tanglit.tangle(text, folder[0].fsPath);
        vscode.window.showInformationMessage(
          `Tanglit: Tangled ${count} file(s)`
        );
      } else if (block) {
        vscode.window.showWarningMessage(
          `Tanglit: Block "${block.tag}" has no export target`
        );
      } else {
        vscode.window.showWarningMessage(
          "Tanglit: Cursor is not inside a code block"
        );
      }
    })
  );
}

async function exportHtml(): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor) return;

  const savePath = await vscode.window.showSaveDialog({
    filters: { HTML: ["html"] },
  });
  if (!savePath) return;

  const html = tanglit.previewHtml(editor.document.getText(), "pico");
  const fs = await import("fs");
  fs.writeFileSync(savePath.fsPath, html);
  vscode.window.showInformationMessage(
    `Tanglit: HTML saved to ${savePath.fsPath}`
  );
}

async function exportSlidesHtml(): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor) return;

  const savePath = await vscode.window.showSaveDialog({
    filters: { HTML: ["html"] },
  });
  if (!savePath) return;

  const html = tanglit.previewSlides(
    editor.document.getText(),
    "black",
    "monokai"
  );
  const fs = await import("fs");
  fs.writeFileSync(savePath.fsPath, html);
  vscode.window.showInformationMessage(
    `Tanglit: Slides HTML saved to ${savePath.fsPath}`
  );
}

async function htmlToPdf(
  html: string,
  outputPath: string,
  landscape: boolean
): Promise<void> {
  const fs = await import("fs");
  const os = await import("os");
  const path = await import("path");

  // Write HTML to temp file
  const tempFile = path.join(os.tmpdir(), "tanglit-export.html");
  fs.writeFileSync(tempFile, html);

  try {
    const puppeteer = await import("puppeteer-core");
    // Try to find Chrome/Chromium
    const browser = await puppeteer.launch({
      headless: true,
      executablePath: findChromePath(),
    });
    const page = await browser.newPage();
    await page.goto(`file://${tempFile}`, { waitUntil: "networkidle0" });
    await page.pdf({
      path: outputPath,
      landscape,
      printBackground: true,
      preferCSSPageSize: true,
    });
    await browser.close();
  } finally {
    fs.unlinkSync(tempFile);
  }
}

function findChromePath(): string {
  const paths = [
    // macOS
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    "/Applications/Chromium.app/Contents/MacOS/Chromium",
    // Linux
    "/usr/bin/google-chrome",
    "/usr/bin/chromium-browser",
    "/usr/bin/chromium",
    // Windows
    "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
  ];

  const fs = require("fs");
  for (const p of paths) {
    if (fs.existsSync(p)) return p;
  }

  throw new Error(
    "Chrome/Chromium not found. Install Chrome or set the path manually."
  );
}
