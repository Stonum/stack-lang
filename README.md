# Extension stack-lang for Visual Studio Code

Adds features for the built-in language of the STACK Software Package  
Based on vscode extension  [buzzzzer.stack](https://marketplace.visualstudio.com/items?itemName=buzzzzer.stack)

---

## Features
- Syntax highlighting
- Code navigation
- Snippets (text templates)
- Formatting the code
- Code validator ( in future )


## Keyboard shortcuts
|Description|Keybinding|
|-|-|
|Replace `stack."Table"` for `~Table~` and backward| Ctrl+Shift+R |
|Replace `[]` for `""`|  |
|List of definitions of the current file| Ctrl+Shift+O |


## Configuration
- `stack.iniPath`: path to stack.ini file. It used for filter loaded folders.
- `stack.lens.enabled`: whether to show CodeLens in Stack files