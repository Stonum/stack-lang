/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import {
   workspace,
   window,
   commands,
   ExtensionContext,
   StatusBarAlignment,
   StatusBarItem,
} from "vscode";

import * as path from "path";

import {
   Executable,
   LanguageClient,
   LanguageClientOptions,
   ServerOptions,
   TransportKind,
} from "vscode-languageclient/node";

import { togglePostgreSQL } from './postgreUtils';

let client: LanguageClient;
let statusBarItem: StatusBarItem;

export async function activate(context: ExtensionContext) {
   const command =
      process.env.SERVER_PATH ||
      path.join(
         context.extensionPath,
         "server",
         process.platform === "linux" ? "stack-lang-server" : "stack-lang-server.exe"
      );

   const run: Executable = {
      command,
      transport: TransportKind.stdio,
      options: {
         env: {
            ...process.env,
            // eslint-disable-next-line @typescript-eslint/naming-convention
            RUST_LOG: "debug",
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
   statusBarItem = window.createStatusBarItem(StatusBarAlignment.Right, 100);
   context.subscriptions.push(statusBarItem);


   client.onNotification(myCommandId, (params) => {
      if (params.text != "") {
         statusBarItem.text = params.text;
         statusBarItem.show();
      } else {
         statusBarItem.hide();
      }
   });

   let disposable = commands.registerCommand('stack.togglePostgreSQL', args => {
      args = args || {}
      const namespace = args.namespace || 'stack'
      const dollar = args.dollar || false
      togglePostgreSQL( namespace, dollar)
   });
   context.subscriptions.push(disposable);

   client.start();
}

export function deactivate(): Thenable<void> | undefined {
   if (!client) {
      return undefined;
   }
   return client.stop();
}
