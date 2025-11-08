# Tanglit — User Guide

Tanglit is a **Markdown-based execution and presentation tool**.  
It lets you **write**, **run**, and **present** Markdown documents with live code blocks, slides, and export source code — all in one place.

## 1. Writing Markdown

Start by writing your content in the main editor. Tanglit fully supports standard **Markdown syntax**, so you can structure your document as you normally would.

You can include:
- Text and **formatting**
- Lists and tables
- `Inline` and fenced code blocks

To insert images stored locally on your computer, use the following syntax with an absolute file path:

```markdown
![image](file:///absolute/path/to/image.png)
```

## 2. Running Code Blocks

Tanglit can execute code blocks directly within your Markdown:

```python
print("Hello from Tanglit!")
```

When you click the play button ▶️, the output appears right below the block.  
You can insert the output back into the Markdown by pressing the **Add to Markdown** button.

## 3. Importing code blocks

Tanglit allows you to **reuse code** across multiple blocks.

In the block metadata, specify the language (`python`) and a block name (`hello_message`):

```python hello_message
hello_message = "Hello from Tanglit!"
```

Then, import that block into another one using the `@[]` syntax:

```python
@[hello_message]
print(hello_message)
```


## 4. Execution Wrappers

Some languages require wrapping the code block with a `main` function.  
For example, in C:

```
#<IMPORTS>#

int main(void) {
    #<BODY>#
    return 0;
}
```

When you execute a C code block, your code is placed inside the `#<BODY>#` placeholder of the template above.

---

If you need to define something **outside** the main function — for example, `#include <stdio.h>` —  
define it in a separate code block and import it using the `use=[]` syntax in the block metadata:

```c stdio
#include <stdio.h>
```

```c use=[stdio]
printf("Hello from Tanglit!");
```

When you run the block above, you’ll see that the message is printed successfully.

> [!NOTE]  
> Learn more about execution wrappers and supported languages in the [Language Configs documentation](./language_configs.md).

## 5. Export Source Code

Tanglit also supports exporting the source code of a block using the `export=filename` syntax in the block metadata:

```python export=hello
@[hello_message]
print(hello_message)
```

When you press the **Tangle code** button in the menu bar below, you’ll be asked to select an output directory.  
After selecting one, all blocks marked with an `export=` flag will be written there as source files.

## 6. Document Generation (PDF/HTML)

You can preview your Tanglit document by pressing the **Preview doc** button in the menu bar below.

To export the entire document as a **PDF** or **HTML**, click **Save doc as PDF** or **Save doc as HTML** in the menu bar below.

> [!WARNING]  
> In the current version, you need to have **Chrome** installed for PDF generation.

## 7. Slide Generation

Tanglit lets you turn any Markdown document into a presentation.  
In fact, this user guide is a presentation itself.

To preview the slides, click **Preview slides** in the menu bar below.

Slides are created automatically from first- and second-level headings (`#` and `##`).

- To start a new slide while keeping the previous title, use `---`.  
- To start a new slide without a title, use `--- ---`.

To export your presentation as a PDF, click **Save slides as PDF** in the menu bar below.


## 8. Exclusion Markers

Since Tanglit allows you to generate both a `PDF/HTML` document and a slide presentation from the same source,  
it provides markers to **hide specific content** depending on the output target.

--- ---
### Excluding Paragraphs

Exclude an entire paragraph from the slides by adding the `&p` marker at the end of the first line.

For example, imagine you write a detailed paragraph that feels too long for a slide, like this one.   &p  
You can hide it from the slide presentation and include a shorter version instead.

To exclude a paragraph from the generated document (but keep it in the slides),  
use the `%p` marker at the end of the first line.

This paragraph will appear **only** in the rendered slides.   %p

To hide a single line instead of a full paragraph,  
use `&` or `%` at the end of the line.

--- ---
### Excluding Lists

Exclude specific **list items** from either the document or the slides using the `&i` and `%i` markers.

- This item will appear in both outputs
- This item is slide-only   %i
- This item is document-only   &i

To exclude an entire list, add `&l` or `%l` at the end of the first item.

- This list is slide-only   %l
- Slide item
- Slide item

-- separator -- %&

- This list is document-only   &l
- Document item
- Document item

--- ---
### Excluding Code Blocks

Exclude code blocks by adding `%` or `&` to the block metadata:

```markdown %
This block will only appear in the slides!
```
```Markdown &
This block will only appear in the document!
```

## 9. Begin Your Tanglit Journey!

With Tanglit, you can turn a simple Markdown file into **source code**, a **reproducible notebook** or a **presentation** — all in one place.

If you prefer working from the terminal, Tanglit also provides a **Command-Line Interface (CLI)**  
that lets you use the backend directly, without the graphical interface.  
See the [CLI documentation](./cli.md) for installation and usage details.
