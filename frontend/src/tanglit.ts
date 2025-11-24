import { invoke } from "@tauri-apps/api/core";

export type ExecutionOutput = {
  stdout: string;
  stderr: string;
  status: number;
};

export type BlockExecute = {
  line?: number | undefined;
  error?: unknown;
  output?: ExecutionOutput;
};

export type Edit = {
  start_line: number;
  end_line: number;
  content: string;
};

enum TANGLIT_COMMANDS {
  exclude = "tanglit_exclude",
  parse_slides = "tanglit_parse_slides",
  parse_blocks = "tanglit_parse_blocks",
  execute = "tanglit_execute_block",
  format_output = "tanglit_format_output",
  gen_slides = "tanglit_gen_slides",
  preview_html = "tanglit_preview_html",
  preview_slides = "tanglit_preview_slides",
  save_pdf = "tanglit_save_pdf",
  save_slides_pdf = "tanglit_save_slides_pdf",
  tangle = "tanglit_tangle",
}

export async function exclude(raw_markdown: string): Promise<string> {
  return await invoke(TANGLIT_COMMANDS.exclude, { raw_markdown });
}

export async function parse_slides(raw_markdown: string): Promise<number[]> {
  const rv = (await invoke(TANGLIT_COMMANDS.parse_slides, { raw_markdown })) as Array<{
    start_line: number;
    tag: string;
  }>;
  return rv.map((item) => item.start_line);
}

export async function parse_blocks(raw_markdown: string) {
  const rv = (await invoke(TANGLIT_COMMANDS.parse_blocks, { raw_markdown })) as Array<{
    end_line: string;
    start_line: number;
    tag: string;
  }>;
  return rv;
}

export async function execute_block(raw_markdown: string, block_name: string): Promise<BlockExecute> {
  try {
    const r = await invoke(TANGLIT_COMMANDS.execute, { raw_markdown, block_name });
    return { output: r as ExecutionOutput };
  } catch (e) {
    return { error: e };
  }
}

export async function gen_slides(raw_markdown: string): Promise<string[]> {
  try {
    const r = (await invoke(TANGLIT_COMMANDS.gen_slides, { raw_markdown })) as string[];
    return r;
  } catch {
    return [];
  }
}

export async function format_output(raw_markdown: string, block_name: string, output: string): Promise<Edit> {
  const r = (await invoke(TANGLIT_COMMANDS.format_output, { raw_markdown, block_name, output })) as Edit;
  return r;
}

export async function preview_html(raw_markdown: string, theme: string) {
  return (await invoke(TANGLIT_COMMANDS.preview_html, { raw_markdown, theme })) as string;
}

export async function preview_slides(raw_markdown: string, theme: string, code_theme: string) {
  console.log("preving slide with thene: ", theme);
  return (await invoke(TANGLIT_COMMANDS.preview_slides, { raw_markdown, theme, code_theme })) as string;
}

export async function save_pdf(raw_markdown: string, theme: string, output_path: string) {
  return (await invoke(TANGLIT_COMMANDS.save_pdf, { raw_markdown, theme, output_path })) as string;
}

export async function tangle(raw_markdown: string, output_dir: string) {
  return (await invoke(TANGLIT_COMMANDS.tangle, { raw_markdown, output_path: output_dir })) as number;
}

export async function save_slides_pdf(raw_markdown: string, theme: string, code_theme: string, output_path: string) {
  return (await invoke(TANGLIT_COMMANDS.save_slides_pdf, { raw_markdown, theme, code_theme, output_path })) as string;
}
