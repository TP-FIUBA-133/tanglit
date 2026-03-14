# Tanglit for VS Code — User Guide

Tanglit for VS Code brings the full power of the Tanglit literate-programming toolkit directly into Visual Studio Code.
You can **write**, **run**, **preview**, and **export** Markdown documents with live code blocks, slides, and source code — all without leaving your editor.

## 1. Installation

Search for **Tanglit** in the VS Code Extensions tab and click **Install**.

Tanglit ships with built-in support for **Python**, **C**, and **Rust** — no configuration needed.

## 2. CodeLens: Run Buttons and Slide Indicators

When you open a Markdown file, Tanglit adds **CodeLens annotations** above your code:

- **▶ Run Block** — appears above each tagged code block. Click to execute it.
- **Slide N** — appears above each `#` and `##` heading, showing the slide number.

These update automatically as you edit the document.

## 3. Running Code Blocks

Click **▶ Run Block** above any tagged code block to execute it.

The output appears in two places:
- **Inline decoration** — a gray summary appears after the code block in the editor.
- **Output channel** — the full output (stdout, stderr, exit code) is written to the **Tanglit** output channel (View → Output → select "Tanglit").

After running a block, two additional CodeLens appear:
- **Add to Markdown** — inserts the output as an `output` code block in your document.
- **Re-run** — executes the block again and updates the decoration.

## 4. Slides Preview

Open the slides preview with any of these methods:
- Click the **preview icon** in the editor title bar (top-right)
- Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`) → **Tanglit: Preview Slides**

A side panel opens showing your Reveal.js slides.
Use the **theme dropdowns** inside the preview panel to change the slide theme and code highlight theme.

The preview updates automatically as you edit (with a 300ms debounce).

## 5. HTML Document Preview

Open the HTML preview with:
- Click the **HTML preview icon** in the editor title bar
- Command Palette → **Tanglit: Preview HTML**

A side panel opens showing the rendered document.
Use the **theme dropdown** to switch between themes (pico, water, sakura, latex).

## 6. Exporting

### Export Menu

Click the **Export icon** in the editor title bar, or use Command Palette → **Tanglit: Export...**

A quick-pick menu appears with all export options:
- **HTML Document** — saves the rendered HTML to a file
- **PDF Document** — generates a PDF using headless Chrome
- **Slides (HTML)** — saves the Reveal.js presentation as HTML
- **Slides (PDF)** — generates a landscape PDF of the slides
- **Tangle All** — exports all code blocks marked with `export=` to files

### Individual Export Commands

You can also run each export command directly from the Command Palette:
- **Tanglit: Export PDF** — prompts for a document theme, then a save location
- **Tanglit: Export Slides PDF** — prompts for slide theme and code theme, then a save location
- **Tanglit: Tangle All** — prompts for an output directory

## 7. Context Menu

Right-click inside a code block to see:
- **Tanglit: Run This Block** — runs the block under the cursor
- **Tanglit: Tangle This Block** — exports the block (if it has an `export=` flag)

## 8. Status Bar

When a Markdown file is open, the status bar (bottom-right) shows:

```
Tanglit: 3 blocks | 5 slides
```

This updates as you edit the document.

## 9. Keyboard Shortcuts

All Tanglit commands are available through the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`).
Type "Tanglit" to see all available commands.

## 10. Tanglit Markdown Syntax

To learn about Tanglit's Markdown features — code block tags, imports (`use=[]`), exports (`export=`), execution wrappers, exclusion markers, and slide generation — see the [Tanglit User Guide](./user_guide.md).

## 11. Advanced: Adding New Languages

Tanglit comes with built-in support for Python, C, and Rust. To add support for a new language, create a TOML configuration file in the Tanglit configuration directory:

```bash
~/.config/tanglit/
```

Or set a custom path via the `TANGLIT_CONFIG_DIR` environment variable.

See the existing language configs for examples of the expected format.

## 13. Requirements

- **Chrome** or **Chromium** must be installed for PDF export (HTML export works without it).
