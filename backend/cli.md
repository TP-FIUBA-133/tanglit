# Using the Backend CLI

```
Usage: backend <COMMAND>

Commands:
  tangle   Tangle a specific code block from a markdown file and export to a file
  exclude  Exclude parts with % markers from input markdown file
  execute  Execute a specific code block from a markdown file and read its output
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Use the `backend` CLI `tangle` subcommand to tangle code blocks from your `.md` files. The `execute` subcommand allows you execute a given supported code block. The relevant build tools or interpreters must be available on your $PATH.

### ✅ Tangle

```sh
backend tangle --input-file-path <INPUT_FILE_PATH> --output-dir <OUTPUT_DIR> --target-block <TARGET_BLOCK>
```

#### 🔧 Options

| Option                     | Description                                                                 |
|---------------------------|-----------------------------------------------------------------------------|
| `--input-file-path`       | Path to the input Markdown file.                                            |
| `--output-dir`            | Directory where the tangled file will be written.                           |
| `--target-block`          | Name (tag) of the code block to tangle.                                     |
| `-h`, `--help`            | Show help message.                                                           |
| `-V`, `--version`         | Show the CLI version.                                                        |

---

### ✅ Execute

```sh
backend execute --input-file-path <INPUT_FILE_PATH> --target-block <TARGET_BLOCK>
```

| Option                     | Description                                                                 |
|---------------------------|-----------------------------------------------------------------------------|
| `--input-file-path`       | Path to the input Markdown file.                                            |
| `--target-block`          | Name (tag) of the code block to execute.                                     |
| `-h`, `--help`            | Show help message.                                                           |
| `-V`, `--version`         | Show the CLI version.                                                        |

---


## 📄 Example Input File

Here’s a simple example of a Markdown file with named code blocks. You can find it under `test_data/test_file.md`:

````markdown
### Test file

This test defines a custom function in one block and uses it in the main block by importing it.

Define headers:

```c headers
#include <stdio.h>
```

Define a helper function in its own block:

```c helper
void greet(const char* name) {
    printf("Hello, %s!\n", name);
}
```

Define main block `main_block`:

```c use=[headers,helper] main_block
    greet("Tangle User");
```
````

## 🧵 Tangling a Block

To generate the full program by resolving all references, run:

```sh
cargo run -- tangle --input-file-path ./test_data/test_file.md --output-dir ./test_data --target-block main_block
```

This will create the file `main_block.c` inside `./test_data/` containing:

````c
#include <stdio.h>

void greet(const char* name) {
    printf("Hello, %s!\n", name);
}

int main() {
    greet("Tangle User");
    return 0;
}

````

## 💻 Executing a Block

You can execute a given bare block of code in your markdown file using the `execute` subcommand and passing the name of the desired block. Interpreted languages are run as-is by the relevant interpreter found in your $PATH variable, while compiled languages like C often need a defined entry point. Blocks from these languages get wrapped in a `main` subroutine/function or equivalent notion and compiled before executing.  
The standard output of the resulting program is captured and shown.


---

## 📌 Tips

- Code block tags (like `headers`, `helper`, or `main_block`) must be unique within the file.
- You can import blocks using `use=[<BLOCK_TAG_1>,<BLOCK_TAG_2>]`.
