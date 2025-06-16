# Using the Backend CLI

Use the `backend` CLI to tangle code blocks from your `.md` files.

### ✅ Basic Usage

```sh
backend --input-file-path <INPUT_FILE_PATH> --output-dir <OUTPUT_DIR> --target-block <TARGET_BLOCK>
```

### 🔧 Options

| Option                     | Description                                                                 |
|---------------------------|-----------------------------------------------------------------------------|
| `--input-file-path`       | Path to the input Markdown file.                                            |
| `--output-dir`            | Directory where the tangled file will be written.                           |
| `--target-block`          | Name (tag) of the code block to tangle.                                     |
| `-h`, `--help`            | Show help message.                                                           |
| `-V`, `--version`         | Show the CLI version.                                                        |

---

## 📄 Example Input File

Here’s a simple example of a Markdown file with named code blocks:

````markdown
### Test file 3

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

Define main block `run`:
```c use=[headers,helper] run
    greet("Tangle User");
```

````

## 🧵 Tangling a Block

To generate the full program by resolving all references, run:

```sh
cargo run --input-file-path example.md --output-dir ./out --target-block run
```

This will create a file inside `./out/` containing:

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

---

## 📌 Tips

- Code block tags (like `header`, `main`, or `full_program`) must be unique within the file.
- You can import block using `use=[block1,block2]`.
