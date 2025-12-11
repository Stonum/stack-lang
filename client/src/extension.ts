/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import {
   workspace,
   window,
   commands,
} from "vscode";
import * as vscode from "vscode";

import * as path from "path";
import { exec } from "child_process";

import {
   Executable,
   LanguageClient,
   LanguageClientOptions,
   ServerOptions,
   TransportKind,
} from "vscode-languageclient/node";

import { togglePostgreSQL, fieldsToPostgreSQL } from './postgreUtils';

let client: LanguageClient;
let statusBarItem: vscode.StatusBarItem;

export async function activate(context: vscode.ExtensionContext) {
   const command =
      process.env.SERVER_PATH ||
      path.join(
         context.extensionPath,
         "server", "bin",
         process.platform,
         process.platform === "linux" ? "stack-lang-server" : "stack-lang-server.exe"
      );
   
   // Make the server executable
   if (process.platform == "linux") {
      await execAsync('chmod +x ' + command);
   }

   const run: Executable = {
      command,
      transport: TransportKind.stdio,
      options: {
         env: {
            ...process.env,
         },
      },
   };
   const serverOptions: ServerOptions = {
      run,
      debug: run,
   };

   // If the extension is launched in debug mode then the debug server options are used
   // Otherwise the run options are used
   // Options to control the language client
   const config = vscode.workspace.getConfiguration('stack');
   let clientOptions: LanguageClientOptions = {
      // Register the server for stack text documents
      documentSelector: [{ scheme: "file", language: "stack" }],
      synchronize: {
         // Notify the server about file changes for files contained in the workspace
         fileEvents: [
            workspace.createFileSystemWatcher("**/*.prg"),
            workspace.createFileSystemWatcher("**/*.hdl"),
         ],
      },
      initializationOptions: {
         lens_enabled: config.get<boolean>('lens.enabled', false)
      }
   };

   // Create the language client and start the client.
   client = new LanguageClient(
      "stack-lang-server",
      "stack lang server",
      serverOptions,
      clientOptions
   );
   
   const myCommandId = 'custom/statusBar';
   // Create a status bar item
   statusBarItem = window.createStatusBarItem(vscode.StatusBarAlignment.Left, 0);
   context.subscriptions.push(statusBarItem);


   client.onNotification(myCommandId, (params) => {
      if (params.text != "") {
         statusBarItem.text = params.text;
         statusBarItem.show();
      } else {
         statusBarItem.hide();
      }
   });

   let togglePSQL = commands.registerCommand('stack.togglePostgreSQL', args => {
      const activeEditor = vscode.window.activeTextEditor;
      const path = activeEditor?.document.uri.path;
      const extension = path?.split('.').pop();

      const namespace = args?.namespace || 'stack';
      const dollar = args?.dollar || extension === 'sql' || false;
      togglePostgreSQL(namespace, dollar);
   });

   context.subscriptions.push(togglePSQL);
   let fieldsToPSQL = commands.registerCommand('stack.fieldsToPostgreSQL', () => {
      fieldsToPostgreSQL()
   });
   context.subscriptions.push(fieldsToPSQL);

   let moveToLine = commands.registerCommand('stack.movetoLine', (line: number) => {
      const editor = window.activeTextEditor;
      if (!editor) return; // No active editor

      // Create a new position at the start of the line
      const position = new vscode.Position(line, 0);

      // Set the selection (collapsed to a single cursor)
      editor.selection = new vscode.Selection(position, position);

      // Optionally, reveal the line in the top of the editor
      editor.revealRange(
         new vscode.Range(position, position),
         vscode.TextEditorRevealType.AtTop
      );
   });
   context.subscriptions.push(moveToLine);

   client.start();
}

export function deactivate(): Thenable<void> | undefined {
   if (!client) {
      return undefined;
   }
   return client.stop();
}

function execAsync (cmd: string) {
   return new Promise((resolve, reject) => {
      exec(cmd, (error, stdout, stderr) => {
         if (error) {
            reject(error);
         }
         resolve(stdout);
      });
   });
};