import * as vscode from "vscode";
import * as tanglit from "./tanglit";
import { TanglitCodeLensProvider } from "./codeLensProvider";
import { SlidesPreviewManager } from "./slidesPreview";
import { HtmlPreviewManager } from "./htmlPreview";
import { ExecutionResultsManager } from "./executionResults";
import { registerCommands } from "./commands";

let statusBarItem: vscode.StatusBarItem;

export function activate(context: vscode.ExtensionContext) {
  tanglit.initConfiguration();

  const codeLensProvider = new TanglitCodeLensProvider();
  const slidesPreview = new SlidesPreviewManager(context);
  const htmlPreview = new HtmlPreviewManager(context);
  const executionResults = new ExecutionResultsManager();

  // Register CodeLens provider for markdown files
  context.subscriptions.push(
    vscode.languages.registerCodeLensProvider(
      { language: "markdown" },
      codeLensProvider
    )
  );

  // Register all commands
  registerCommands(
    context,
    codeLensProvider,
    slidesPreview,
    htmlPreview,
    executionResults
  );

  // Status bar
  statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    100
  );
  context.subscriptions.push(statusBarItem);
  updateStatusBar(vscode.window.activeTextEditor);

  // Debounced document change listener
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;
  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => {
      if (e.document.languageId !== "markdown") {
        return;
      }
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
      debounceTimer = setTimeout(() => {
        codeLensProvider.refresh();
        slidesPreview.update(e.document);
        htmlPreview.update(e.document);
        updateStatusBar(vscode.window.activeTextEditor);
      }, 300);
    })
  );

  // Update status bar when active editor changes
  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      updateStatusBar(editor);
    })
  );
}

function updateStatusBar(editor: vscode.TextEditor | undefined) {
  if (!editor || editor.document.languageId !== "markdown") {
    statusBarItem.hide();
    return;
  }
  const text = editor.document.getText();
  const blocks = tanglit.parseBlocks(text);
  const slides = tanglit.parseSlides(text);
  statusBarItem.text = `Tanglit: ${blocks.length} blocks | ${slides.length} slides`;
  statusBarItem.show();
}

export function deactivate() {}
