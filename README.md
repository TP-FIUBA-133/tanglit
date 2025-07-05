# Tanglit  
*A literate‑programming toolkit that unifies docs, executes code, generates slides and exports code from a single source.*

# Project Roadmap

## Backend Development

### ✅ Milestone 1: Basic Code Export
- [x] Integrate Markdown parser  
- [x] Parse code blocks from Markdown files  
- [x] Build basic CLI interface  
- [x] Enable CI  

### ✅ Milestone 2: Tag-Based Code Export
- [x] Add support for block tags  
- [x] Import referenced code blocks via metadata (`use=[block1, block2]`)  
- [x] Resolve macros within code blocks (`@[block1]`)  

### 🚧 Milestone 3: Block Execution
- [ ] Implement block execution (support for `C` and `Python`) 🔨  
- [ ] Make compilation options customizable  
- [ ] Make the `main` block structure customizable  

### ✅ Milestone 4: Exclusion Support
- [x] Exclude items from Markdown output  
- [x] Exclude items from slide output  

### 🧩 Milestone 5: Slides Generation
- [x] Parse slide delimiters  
- [x] Set slide titles correctly  
- [ ] Implement PDF slide generation  

### 🎨 Milestone 6: Document Rendering
- [ ] Develop Markdown-to-HTML converter  
- [ ] Implement styling (fonts, layouts, etc.)  
- [ ] Add weave functionality (combine rendered doc with code output)  

## 💻 Frontend Development

### 🚧 Milestone 1: Foundation
- [x] Research frameworks:
  - HTMX  
  - Electron  
  - React  
  - VS Code API / LSP  
- [ ] Build minimal editor interface 🔨  
