# Tanglit CLI

The Tanglit Command-Line Interface (`tanglit`) lets you run core Tanglit features directly from the terminal — such as executing code blocks, generating documents, or creating slides.

---

## 📖 Usage

```bash
tanglit <COMMAND> [OPTIONS]
```

To list available commands:

```bash
tanglit help
```

---

## 🧩 Global Options

| Flag | Description |
|------|--------------|
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

---

## ⚙️ Commands Overview

| Command | Description |
|----------|--------------|
| [`tangle`](#tangle) | Tangle a specific code block from a Markdown file and export it |
| [`execute`](#execute) | Execute a specific code block and display its output |
| [`tangle-all`](#tangle-all) | Tangle and export all marked code blocks from a Markdown file |
| [`generate-pdf`](#generate-pdf) | Generate a PDF from a Markdown file |
| [`generate-html`](#generate-html) | Generate an HTML document from a Markdown file |
| [`generate-slides-md`](#generate-slides-md) | Generate Markdown slides |
| [`generate-slides-pdf`](#generate-slides-pdf) | Generate PDF slides |
| [`help`](#help) | Print help for commands |

---

## 🧱 Command Reference

### `tangle`

Tangle a specific code block from a Markdown file and export it to a file.

**Usage:**
```bash
tanglit tangle --output-dir <OUTPUT_DIR> --target-block <TARGET_BLOCK> <INPUT_FILE_PATH>
```

**Options:**
```
  -h, --help     Print help
  -V, --version  Print version
```

**Arguments:**
```
  -o, --output-dir <OUTPUT_DIR>           Path to the directory where output files will be saved. [env: OUTPUT_DIR=]
  -t, --target-block <TARGET_BLOCK>       Tag of the code block to tangle. [env: TARGET_BLOCK=]
  <INPUT_FILE_PATH>                       Path to the input Markdown file. [env: INPUT_FILE_PATH=]
```

---

### `execute`

Execute a specific code block from a Markdown file and display its output.

**Usage:**
```bash
tanglit execute --target-block <TARGET_BLOCK> <INPUT_FILE_PATH>
```

**Options:**
```
  -h, --help     Print help
  -V, --version  Print version
```

**Arguments:**
```
  -t, --target-block <TARGET_BLOCK>       Tag of the code block to execute. [env: TARGET_BLOCK=]
  <INPUT_FILE_PATH>                       Path to the input Markdown file. [env: INPUT_FILE_PATH=]
```

---

### `tangle-all`

Tangle and export all code blocks from a Markdown file.

**Usage:**
```bash
tanglit tangle-all --output-dir <OUTPUT_DIR> <INPUT_FILE_PATH>
```

**Options:**
```
  -h, --help     Print help
  -V, --version  Print version
```

**Arguments:**
```
  -o, --output-dir <OUTPUT_DIR>           Path to the directory where output files will be saved. [env: OUTPUT_DIR=]
  <INPUT_FILE_PATH>                       Path to the input Markdown file. [env: INPUT_FILE_PATH=]
```

---

### `generate-pdf`

Generate a PDF document from a Markdown file, skipping `%`-marked items.

**Usage:**
```bash
tanglit generate-pdf --output-file <OUTPUT_FILE_PATH> <INPUT_FILE_PATH>
```

**Options:**
```
  -h, --help     Print help
  -V, --version  Print version
```

**Arguments:**
```
  -o, --output-file <OUTPUT_FILE_PATH>    Path to the output file. [env: OUTPUT_FILE_PATH=]
  <INPUT_FILE_PATH>                       Path to the input Markdown file. [env: INPUT_FILE_PATH=]
```

---

### `generate-html`

Generate an HTML document from a Markdown file, skipping `%`-marked items.

**Usage:**
```bash
tanglit generate-html --output-file <OUTPUT_FILE_PATH> <INPUT_FILE_PATH>
```

**Options:**
```
  -h, --help     Print help
  -V, --version  Print version
```

**Arguments:**
```
  -o, --output-file <OUTPUT_FILE_PATH>    Path to the output file. [env: OUTPUT_FILE_PATH=]
  <INPUT_FILE_PATH>                       Path to the input Markdown file. [env: INPUT_FILE_PATH=]
```

---

### `generate-slides-md`

Generate Markdown slides from a Markdown file.

**Usage:**
```bash
tanglit generate-slides-md --output-file <OUTPUT_FILE_PATH> <INPUT_FILE_PATH>
```

**Options:**
```
  -h, --help     Print help
  -V, --version  Print version
```

**Arguments:**
```
  -o, --output-file <OUTPUT_FILE_PATH>    Path to the output file. [env: OUTPUT_FILE_PATH=]
  <INPUT_FILE_PATH>                       Path to the input Markdown file. [env: INPUT_FILE_PATH=]
```

---

### `generate-slides-pdf`

Generate PDF slides from a Markdown file.

**Usage:**
```bash
tanglit generate-slides-pdf --output-file <OUTPUT_FILE_PATH> <INPUT_FILE_PATH>
```

**Options:**
```
  -h, --help     Print help
  -V, --version  Print version
```

**Arguments:**
```
  -o, --output-file <OUTPUT_FILE_PATH>    Path to the output file. [env: OUTPUT_FILE_PATH=]
  <INPUT_FILE_PATH>                       Path to the input Markdown file. [env: INPUT_FILE_PATH=]
```

---

### `help`

Print help for the CLI or a given subcommand.

**Usage:**
```bash
tanglit help [COMMAND]
```
