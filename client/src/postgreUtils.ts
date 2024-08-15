import * as vscode from 'vscode'

const toPostgreRegExp = /~\${0,1}([^~]*)~/g

export function togglePostgreSQL(
   namespace: string = 'stack',
   addDollar: boolean = false,
): void {
   const editor = vscode.window.activeTextEditor
   if (!editor) {
      return
   }

   let hasTilda = false
   forAllSelections(editor, (r, t) => {
      hasTilda = hasTilda || t.replace(toPostgreRegExp, '') !== t
   })

   if (hasTilda) {
      convertToPostgreSQL(namespace)
   } else {
      convertFromPostgreSQL(namespace, addDollar)
   }
}

function convertFromPostgreSQL(
   namespace: string = 'stack',
   addDollar: boolean = false,
): void {
   const editor = vscode.window.activeTextEditor
   if (!editor) {
      return
   }

   editor.edit(b => {
      const re1 = new RegExp(namespace + '\\."([^"]*)"', 'g')
      const re2 = new RegExp(namespace + '\\.([^\\s\\(]*)', 'g')
      const repl = addDollar ? '~$$$1~' : '~$1~'

      forAllSelections(editor, (range, text) => {
         b.replace(range, text.replace(re1, repl).replace(re2, repl))
      })
   })
}

function convertToPostgreSQL(namespace: string = 'stack'): void {
   const editor = vscode.window.activeTextEditor
   if (!editor) {
      return
   }

   editor.edit(b => {
      const repl = namespace + '."$1"'

      forAllSelections(editor, (range, text) => {
         b.replace(range, text.replace(toPostgreRegExp, repl))
      })
   })
}



function fieldsToPostgreSQL(): void {
   const editor = vscode.window.activeTextEditor
   if (!editor) {
      return
   }

   editor.edit(b => {
      forAllSelections(editor, (range, text) => {
         b.replace(range, text.replace(/[\[\]]/g, '"'))
      })
   })
}

function forAllSelections(
   editor: vscode.TextEditor,
   func: (range: vscode.Range, text: string) => void,
): void {
   if (editor.selections.length === 1 && editor.selections[0].isEmpty) {
      func(
         new vscode.Range(
            new vscode.Position(0, 0),
            editor.document.lineAt(editor.document.lineCount - 1).range.end,
         ),
         editor.document.getText(),
      )
   } else {
      for (let i = 0; i < editor.selections.length; i++) {
         func(editor.selections[i], editor.document.getText(editor.selections[i]))
      }
   }
}
