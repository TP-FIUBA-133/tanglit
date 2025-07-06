# Tanglit Editor

## How to run (dev mode)

1. Install the dependencies:
   ```bash
   npm install
   ```
2. Start the development server:
   ```bash
    make run-dev
    ```

In the left panel, we have the markdown editor.
In the gutter (icons next to the line numbers) we have the following icons:
- In the lines where a new slide starts, we have the slide number (yellow number).
- In the lines where a new code block starts, we have a Run block button (green play icon).
  - When you click this button, the block will be run and the output is displayed.

In the right panel, we have the markdown with the slide exclusions already processed, so you won't see the 
content marked with %, %p, %l, %i.

Sample markdown button loads a sample markdown file to the editor.
Open button opens a file dialog to select a markdown file to edit.
The rest of the buttons do nothing at all for now.