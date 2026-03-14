// Native addon bindings for the tanglit Rust backend
// Load the platform-specific native addon
// Local dev builds produce tanglit.node, CI builds produce tanglit.<platform>.node
import * as os from "os";
import * as path from "path";

function loadNativeAddon() {
  const nativeDir = path.join(__dirname, "..", "native");
  // Try the generic name first (local dev builds)
  try {
    return require(path.join(nativeDir, "tanglit.node"));
  } catch {
    // Fall back to platform-specific name (CI builds)
    const platform = os.platform();
    const arch = os.arch();
    const platformMap: Record<string, string> = {
      "darwin-arm64": "tanglit.darwin-arm64.node",
      "darwin-x64": "tanglit.darwin-x64.node",
      "linux-x64": "tanglit.linux-x64-gnu.node",
      "win32-x64": "tanglit.win32-x64-msvc.node",
    };
    const filename = platformMap[`${platform}-${arch}`];
    if (!filename) {
      throw new Error(`Unsupported platform: ${platform}-${arch}`);
    }
    return require(path.join(nativeDir, filename));
  }
}

const native = loadNativeAddon();

export interface CodeBlock {
  tag: string;
  language: string | null;
  code: string;
  imports: string[];
  export: string | null;
  startLine: number;
  endLine: number;
}

export interface SlideByIndex {
  startLine: number;
}

export interface ExecutionOutput {
  stdout: string;
  stderr: string;
  status: number | null;
}

export interface Edit {
  content: string;
  startLine: number;
  endLine: number;
}

export function initConfiguration(): void {
  native.initConfiguration();
}

export function parseBlocks(rawMarkdown: string): CodeBlock[] {
  return native.parseBlocks(rawMarkdown);
}

export function parseSlides(rawMarkdown: string): SlideByIndex[] {
  return native.parseSlides(rawMarkdown);
}

export function executeBlock(
  rawMarkdown: string,
  blockName: string
): ExecutionOutput {
  return native.executeBlock(rawMarkdown, blockName);
}

export function formatOutput(
  rawMarkdown: string,
  blockName: string,
  output: ExecutionOutput
): Edit {
  return native.formatOutput(rawMarkdown, blockName, output);
}

export function previewHtml(rawMarkdown: string, theme: string): string {
  return native.previewHtml(rawMarkdown, theme);
}

export function previewSlides(
  rawMarkdown: string,
  theme: string,
  codeTheme: string
): string {
  return native.previewSlides(rawMarkdown, theme, codeTheme);
}

export function tangle(rawMarkdown: string, outputPath: string): number {
  return native.tangle(rawMarkdown, outputPath);
}

export function exclude(rawMarkdown: string): string {
  return native.exclude(rawMarkdown);
}

export function savePdf(
  rawMarkdown: string,
  theme: string,
  outputPath: string
): void {
  native.savePdf(rawMarkdown, theme, outputPath);
}

export function saveSlidesPdf(
  rawMarkdown: string,
  theme: string,
  codeTheme: string,
  outputPath: string
): void {
  native.saveSlidesPdf(rawMarkdown, theme, codeTheme, outputPath);
}
