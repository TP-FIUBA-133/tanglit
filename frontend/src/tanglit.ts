import { invoke } from "@tauri-apps/api/core";

export type ExecutionOutput = {
  stdout: string;
  stderr: string;
  status: number;
};

export type BlockExecute = {
  error?: unknown;
  result?: ExecutionOutput;
};

enum TANGLIT_COMMANDS {
  exclude = "tanglit_exclude",
  parse_slides = "tanglit_parse_slides",
  parse_blocks = "tanglit_parse_blocks",
  execute = "tanglit_execute_block",
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
    start_line: number;
    tag: string;
  }>;
  return rv;
}

export async function execute_block(raw_markdown: string, block_name: string): Promise<BlockExecute> {
  try {
    const r = await invoke(TANGLIT_COMMANDS.execute, { raw_markdown, block_name });
    return { result: r as ExecutionOutput };
  } catch (e) {
    return { error: e };
  }
}
