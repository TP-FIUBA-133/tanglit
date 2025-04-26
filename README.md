# Tanglit
A literate‑programming toolkit that unifies docs, executes code, generates slides and exports code from a single source. 

# Project Roadmap

## Backend Development

### Milestone 1: Basic Code Export
- [ ] Initialize Rust project
- [ ] Implement block parsing and concatenation
- [ ] Integrate Markdown parser
- [ ] Build basic CLI interface (using [clap](https://github.com/clap-rs/clap)

### Milestone 2: Tag-Based Code Export
- [ ] Add support for block tags (`@block1`, `@block2`)
- [ ] Implement tag-based block sorting and export

### Milestone 3: Document Rendering
- [ ] Develop Markdown-to-HTML converter
- [ ] Implement styling (fonts, layouts, etc.)
- [ ] Add weave functionality

### Milestone 4: Block Execution
- [ ] Add unique identifiers to blocks
- [ ] Implement single-block execution
- [ ] Return execution output

### Milestone 5: Advanced Execution
- [ ] Add macro support to block execution

### Milestone 6: Slides Generation
- [ ] Parse slide delimiters
- [ ] Implement PDF slide generation

## Frontend Development

### Milestone 1: Foundation
- [ ] Research frameworks:
  - HTMX
  - Electron
  - React
  - VS Code API/LSP
- [ ] Build minimal editor interface
