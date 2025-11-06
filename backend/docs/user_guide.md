# Tanglit — User Guide

Tanglit is a **Markdown-based execution and presentation tool**.  
It lets you **write**, **run**, and **present** Markdown documents with live code blocks, slides, and export source code — all in one place.

## 1. Writing Markdown

Write your content in the main editor. Tanglit supports standard Markdown syntax.

You can include:
- **Text**
- Lists
- `Code`
- Images

## 2. Running Code Blocks

Tanglit can execute code blocks directly within your Markdown:

```python
print("Hello from Tanglit!")
```

When you click the play button ▶️, the output appears right below the block.
You can insert the output back into the Markdown by pressing the `Add to Markdown` button.

## 3. Importing code blocks

You can also import code from other blocks.

In the block metadata, specify a language (`python`) and a block name (`hello_message`).

```python hello_message
hello_message = "Hello from Tanglit!"
```

Then, import the block using the `@[]` syntax.

```python
@[hello_message]
print(hello_message)
```

## 4. Execution wrappers

Some languages require wrapping the code block with a main function.
For example in C:

```
#<IMPORTS>#

int main(void){
    #<BODY>#
    return 0;
}
```

When you execute a C code block, your code is placed inside the `#<BODY>#` placeholder of the template above.

---

But what if you need to define something outside the main function — for example, `#include <stdio.h>`?
In that case, define everything you want outside the wrapper in a separate code block, and import it using the `use=[]` syntax in the block metadata:

```c stdio
#include <stdio.h>
```

```c use=[stdio]
printf("Hello from Tanglit!");
```

If you run the block above, you'll see that the message is printed successfully ✅

> [!IMPORTANT]  
> We provide default templates for our default supported languages (currently `C`, `Python` and `Rust`).  
> See our [language config docs]() for more.

## 5. Export source code

Tanglit also supports exporting the source code of a block using the `export=filename` syntax in the block metadata:

```python export=hello
@[hello_message]
print(hello_message)
```

When you press the `Tangle code` button in the menu bar below, you will be asked to select an output directory.
After selecting one, all blocks marked with an `export=` flag will be written there as source files.

## 6. Document Generation (PDF/HTML)

You can preview your Tanglit document by pressing the **Preview doc** button in the menu bar below.

To export the entire document as a **PDF** or **HTML**, press **Save doc as PDF** or **Save doc as HTML** in the menu bar below.

> [!WARNING]  
> In the current version, you need to have **Chrome** installed for PDF generation.


## 7. Slide Generation

Tanglit lets you turn any Markdown document into a presentation.  
To preview your slides, click **Preview slides** in the menu bar below.

Slides are created automatically from first- and second-level headings (`#` and `##`).

- To start a new slide while keeping the previous title, use `---`.  
- To start a new slide without a title, use `--- ---`.

To export your presentation as a PDF, click **Save slides as PDF** in the menu bar below.


## 8. Excluding markers
